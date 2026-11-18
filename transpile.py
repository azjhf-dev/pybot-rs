#!/usr/bin/env python3
"""Transpile PyBot's bot/ package (pure-rule Python) to Rust for pybot-rs.

Conventions (see src/pyrt/):
  - Every Python function -> `pub fn name(args: &[Py]) -> Py`; keyword args arrive
    as a trailing Py::Kwargs and the prologue sorts them into parameters.
  - Module-level names -> `static NAME: Lazy<RwLock<Py>>` + accessors g_NAME()/s_NAME().
  - Module bodies run once via ensure_init() (Once-guarded).
  - Python exceptions are Rust panics carrying PyExc; try/except -> catch().
"""
import ast
import hashlib
import os
import re
import symtable as symtable_mod
import sys

SRC = os.path.expanduser("~/pybot-job/upstream/bot")
DST = os.path.expanduser("~/pybot-job/pybot-rs/src/gen")
CRATES = os.path.expanduser("~/pybot-job/pybot-rs/crates")

RUST_KEYWORDS = set("""as break const continue crate dyn else enum extern false fn for if impl in let loop
match mod move mut pub ref return self Self static struct super trait true type unsafe use where while
async await box abstract become do final macro override priv typeof unsized virtual yield try gen""".split())

def rust_ident(name):
    name = name.replace('-', '_')
    if name == '_':
        return '_u'
    if name in RUST_KEYWORDS:
        return name + "_"
    if name and name[0].isdigit():
        name = "_" + name
    return name

def rust_string(s):
    """Render a Python str as a Rust string literal."""
    out = []
    for c in s:
        if c == '"': out.append('\\"')
        elif c == '\\': out.append('\\\\')
        elif c == '\n': out.append('\\n')
        elif c == '\r': out.append('\\r')
        elif c == '\t': out.append('\\t')
        elif c == '\0': out.append('\\0')
        elif ord(c) < 0x20 or ord(c) == 0x7f: out.append('\\u{%02x}' % ord(c))
        else: out.append(c)
    return '"' + ''.join(out) + '"'

# ---------------------------------------------------------------- module discovery
STDLIB = {"re", "random", "datetime", "string", "copy", "math", "sys", "os", "itertools",
          "functools", "unicodedata", "hashlib", "difflib", "importlib", "pkgutil", "operator",
          "ast", "json", "textwrap", "html", "time", "calendar", "decimal", "fractions", "io",
          "collections", "enum", "typing", "abc", "contextlib", "warnings", "base64", "binascii"}

def discover():
    mods = {}  # dotted name (no bot. prefix) -> file path
    for root, _, files in os.walk(SRC):
        for f in files:
            if not f.endswith('.py'):
                continue
            path = os.path.join(root, f)
            rel = os.path.relpath(path, SRC)[:-3]
            parts = rel.split(os.sep)
            if parts[-1] == '__init__':
                parts = parts[:-1]
            dotted = '.'.join(parts)
            if dotted == '':
                continue
            mods[dotted] = path
    return mods

PKG_ORDER = {"understanding": 0, "memory": 1, "knowledge": 2, "compose": 3, "modules": 4,
             "runtime": 5, "__main__": 5}
CORE_PKGS = {"runtime", "__main__"}

def pkg_of(dotted):
    return dotted.split('.')[0]

def crate_of(dotted):
    return 'gen_' + dotted.replace('.', '_')

def rust_path(dotted, cur_dotted=None):
    """Absolute in-crate or cross-crate path for a dotted bot module."""
    parts = '::'.join(rust_ident(p) for p in dotted.split('.'))
    if cur_dotted is not None and crate_of(dotted) == crate_of(cur_dotted):
        return 'crate::' + parts
    return crate_of(dotted) + '::' + parts

def out_file(dotted):
    """crates/<crate>/src/<pkg>/<rest>.rs"""
    parts = [rust_ident(p) for p in dotted.split('.')]
    crate = crate_of(dotted)
    base = os.path.join(CRATES, crate, 'src')
    if len(parts) == 1:
        return os.path.join(base, parts[0] + '.rs')
    return os.path.join(base, *parts[:-1], parts[-1] + '.rs')

# ---------------------------------------------------------------- import analysis
class ImportInfo:
    """alias -> resolved thing for one source file."""
    def __init__(self):
        self.func = {}     # alias -> (rust module path, fn name)   [direct function]
        self.glob = {}     # alias -> (rust module path, static name)
        self.mod = {}      # alias -> rust module path (bot module)
        self.std = {}      # alias -> stdlib module name
        self.cls = {}      # alias -> (rust module path, class name)
        self.unknown = {}  # alias -> (module, name)

def resolve_imports(tree, dotted, mods, classes):
    info = ImportInfo()
    def visit(node):
        resolve_one_import(node, info, dotted, mods, classes)
        if not isinstance(node, (ast.FunctionDef, ast.ClassDef, ast.Lambda)):
            for child in ast.iter_child_nodes(node): visit(child)
    for node in tree.body: visit(node)
    return info

def resolve_one_import(node, info, dotted, mods, classes):
    if True:
        if isinstance(node, ast.Import):
            for a in node.names:
                name = a.name
                alias = a.asname or name.split('.')[0]
                top = name.split('.')[0]
                if top in STDLIB:
                    info.std[alias] = name
                elif top == 'bot':
                    modname = name[4:]
                    info.mod[alias] = modname
        elif isinstance(node, ast.ImportFrom):
            module = node.module or ''
            level = node.level or 0
            base = dotted.split('.')
            # current package: dotted minus the module itself (or minus nothing for __init__ path)
            src = mods[dotted]
            is_init = src.endswith('__init__.py')
            pkg = base if is_init else base[:-1]
            if level:
                target_pkg = pkg[:len(pkg) - (level - 1)] if level - 1 <= len(pkg) else []
            else:
                target_pkg = []
            full = '.'.join(target_pkg + ([module] if module else [])) if level else module
            top = full.split('.')[0] if full else ''
            if not level and top in STDLIB:
                for a in node.names:
                    alias = a.asname or a.name
                    info.unknown[alias] = (full, a.name)
                return
            if level == 0 and top == 'bot':
                full = full[4:]
            if full in mods and not mods[full].endswith('__init__.py'):
                for a in node.names:
                    alias = a.asname or a.name
                    if a.name in classes.get(full, set()):
                        info.cls[alias] = (full, a.name)
                    else:
                        # function or global: decided at codegen via def sets
                        info.func[alias] = (full, a.name)
            elif full in mods:
                # package __init__: names live in mod.rs
                for a in node.names:
                    alias = a.asname or a.name
                    sub = full + '.' + a.name
                    if sub in mods:
                        info.mod[alias] = sub
                    else:
                        info.func[alias] = (full, a.name)
            else:
                # `from . import submodule` when full resolves deeper, or unknown
                for a in node.names:
                    alias = a.asname or a.name
                    sub = (full + '.' + a.name) if full else a.name
                    if sub in mods:
                        info.mod[alias] = sub
                    else:
                        info.unknown[alias] = (full, a.name)

def find_classes(mods):
    out = {}
    for dotted, path in mods.items():
        try:
            tree = ast.parse(open(path).read())
        except Exception:
            continue
        out[dotted] = {n.name for n in tree.body if isinstance(n, ast.ClassDef)}
    return out

def find_defs(mods):
    """dotted -> (functions, globals) at module level."""
    fns, glbs = {}, {}
    for dotted, path in mods.items():
        try:
            tree = ast.parse(open(path).read())
        except Exception:
            continue
        fns[dotted] = set()
        glbs[dotted] = set()
        for n in tree.body:
            if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef)):
                fns[dotted].add(n.name)
            elif isinstance(n, ast.ClassDef):
                fns[dotted].add(n.name)  # constructor
            elif isinstance(n, ast.Assign):
                for t in n.targets:
                    for name in target_names(t):
                        glbs[dotted].add(name)
            elif isinstance(n, (ast.AnnAssign, ast.AugAssign)) and isinstance(n.target, ast.Name):
                glbs[dotted].add(n.target.id)
            elif isinstance(n, (ast.If, ast.For, ast.While, ast.Try)):
                for name in assigned_names(n):
                    glbs[dotted].add(name)
    return fns, glbs

def target_names(t):
    if isinstance(t, ast.Name):
        return [t.id]
    if isinstance(t, (ast.Tuple, ast.List)):
        return [n for e in t.elts for n in target_names(e)]
    if isinstance(t, ast.Starred):
        return target_names(t.value)
    return []

def is_generator(node):
    def visit(n):
        if isinstance(n, (ast.Yield, ast.YieldFrom)): return True
        if isinstance(n, (ast.FunctionDef, ast.Lambda, ast.ClassDef)): return False
        return any(visit(c) for c in ast.iter_child_nodes(n))
    return any(visit(c) for c in node.body)

def assigned_names(node):
    out = []
    for n in ast.walk(node):
        if isinstance(n, ast.Name) and isinstance(n.ctx, (ast.Store,)):
            out.append(n.id)
        elif isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            out.append(n.name)
    return out

# ---------------------------------------------------------------- codegen
HELPERS = "use pyrt::builtins::*;\nuse pyrt::pybuiltins::*;\nuse pyrt::stdlib::*;\nuse pyrt::value::*;\nuse pyrt::exc;\nuse pyrt::{pystr, pyre, pyrandom, pydatetime};\nuse std::sync::{LazyLock as Lazy, RwLock, Once};\nuse std::cell::{Cell, RefCell};\nuse std::rc::Rc;\nuse indexmap::IndexMap;\n\n"

class Ctx:
    """One function or module-body scope."""
    def __init__(self, gen, kind, name):
        self.gen = gen
        self.kind = kind            # 'fn' | 'module'
        self.name = name
        self.locals = set()         # names declared as Rust locals
        self.params = []
        self.freevars = set()       # captured from enclosing (for closures)
        self.global_names = set()   # declared `global x`
        self.parent = None
        self.comp_body = None
        self.tmp = 0

    def tmpname(self, base="t"):
        self.tmp += 1
        return f"__{base}{self.tmp}"

class Gen:
    def __init__(self, dotted, path, mods, fns, glbs, classes):
        self.dotted = dotted
        self.path = path
        self.tree = ast.parse(open(path).read())
        self.mods = mods
        self.fns = fns
        self.glbs = glbs
        self.classes = classes
        self.info = resolve_imports(self.tree, dotted, mods, classes)
        self.src = open(path).read()
        self.warnings = []
        self.temp = 0
        # symtable for scopes
        try:
            self.st = symtable_mod.symtable(self.src, path, 'exec')
        except SyntaxError:
            self.st = None
        self.decorated = set()   # decorated top-level fns: name stays a Py value
        self.lines = []

    def mpath(self, target_dotted):
        return rust_path(target_dotted, self.dotted)

    def is_xref(self, target_dotted):
        return target_dotted != self.dotted

    def ref_fn_value(self, tdotted, item):
        """Py value referencing a fn/class/decorated in another module."""
        r = rust_ident(item)
        if self.is_xref(tdotted):
            return f'py_func({rust_string(item)}, |args: &[Py]| pyrt::xrefs::xcall({rust_string(tdotted)}, {rust_string(item)}, args))'
        mp = self.mpath(tdotted)
        if item in self.decorated_set(tdotted):
            return f'{mp}::g_{r}()'
        return f'py_func({rust_string(item)}, |args: &[Py]| {mp}::{r}(args))'

    def ref_global_read(self, tdotted, item):
        r = rust_ident(item)
        if self.is_xref(tdotted):
            return f'pyrt::xrefs::xget({rust_string(tdotted)}, {rust_string(item)})'
        return f'{self.mpath(tdotted)}::g_{r}()'

    def ref_module(self, tdotted):
        if self.is_xref(tdotted):
            return f'pyrt::xrefs::xmodule({rust_string(tdotted)})'
        return f'{self.mpath(tdotted)}::module_obj()'

    def ref_builder(self, tdotted, item):
        """call builder for call_with_args targeting a fn/class in another module."""
        r = rust_ident(item)
        if self.is_xref(tdotted):
            return f'|args: &[Py]| pyrt::xrefs::xcall({rust_string(tdotted)}, {rust_string(item)}, args)'
        mp = self.mpath(tdotted)
        if item in self.decorated_set(tdotted):
            return f'|args: &[Py]| py_call(&{mp}::g_{r}(), args)'
        return f'|args: &[Py]| {mp}::{r}(args)'

    def warn(self, msg, node=None):
        line = getattr(node, 'lineno', '?')
        self.warnings.append(f"{self.dotted}:{line}: {msg}")

    # ------------------------------------------------------------ scopes
    def fn_scope(self, node):
        """Names that are local to this Python function (params + assigned + nested defs).
        Does not descend into nested function/class/lambda scopes."""
        locals_ = set()
        for a in node.args.args + node.args.kwonlyargs:
            locals_.add(a.arg)
        if node.args.vararg:
            locals_.add(node.args.vararg.arg)
        if node.args.kwarg:
            locals_.add(node.args.kwarg.arg)
        globals_declared = set()
        for n in ast.walk(node):
            if isinstance(n, ast.Global):
                globals_declared.update(n.names)

        def walk(n):
            if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef)):
                locals_.add(n.name)
                # decorator/defaults are evaluated in this scope
                for d in n.decorator_list:
                    walk(d)
                for d in n.args.defaults:
                    walk(d)
                return
            if isinstance(n, ast.ClassDef):
                locals_.add(n.name)
                return
            if isinstance(n, ast.Lambda):
                for d in n.args.defaults:
                    walk(d)
                return
            if isinstance(n, (ast.Import, ast.ImportFrom)):
                locals_.update(a.asname or (a.name.split('.')[0] if isinstance(n, ast.Import) else a.name) for a in n.names)
            if isinstance(n, ast.Name) and isinstance(n.ctx, ast.Store):
                locals_.add(n.id)
            elif isinstance(n, (ast.For,)):
                locals_.update(target_names(n.target))
            elif isinstance(n, ast.ExceptHandler) and n.name:
                locals_.add(n.name)
            elif isinstance(n, ast.Assign):
                for t in n.targets:
                    locals_.update(target_names(t))
            elif isinstance(n, ast.AnnAssign):
                locals_.update(target_names(n.target))
            elif isinstance(n, ast.AugAssign):
                locals_.update(target_names(n.target))
            elif isinstance(n, ast.NamedExpr):
                locals_.update(target_names(n.target))
            for c in ast.iter_child_nodes(n):
                walk(c)

        for c in ast.iter_child_nodes(node):
            walk(c)
        return locals_ - globals_declared, globals_declared

    def emit(self, s):
        self.lines.append(s)

    # ------------------------------------------------------------ entry
    def generate(self):
        self.emit("#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]")
        self.emit(HELPERS)
        # statics for module-level names
        body = self.tree.body
        mod_globals = self.glbs.get(self.dotted, set()) | set()
        # decorated functions and plain functions
        fdefs = [n for n in body if isinstance(n, ast.FunctionDef)]
        cdefs = [n for n in body if isinstance(n, ast.ClassDef)]
        seen = {}
        for f in reversed(fdefs):
            old = f.name
            if old in seen:
                seen[old] += 1
                f.name += '__dup' + str(seen[old])
            else:
                seen[old] = 0
        fn_names = {f.name for f in fdefs} | {c.name for c in cdefs}
        # `from M import G as A` at module level binds A as a module global
        self.import_globals = {}
        for alias, pair in self.info.func.items():
            if alias not in fn_names:
                self.import_globals[alias] = pair
        mod_globals |= set(self.import_globals)
        for name in sorted(mod_globals):
            rname = rust_ident(name)
            self.emit(f"thread_local! {{ static G_{rname}: RefCell<Py> = const {{ RefCell::new(Py::None) }}; }}")
        # decorated functions are bound as globals (Py values)
        self.rebound = {f.name for f in fdefs if f.name in mod_globals}
        for f in fdefs:
            if f.decorator_list or f.name in self.rebound:
                self.decorated.add(f.name)
                rname = rust_ident(f.name)
                if rname not in {rust_ident(x) for x in mod_globals}:
                    self.emit(f"thread_local! {{ static G_{rname}: RefCell<Py> = const {{ RefCell::new(Py::None) }}; }}")
        # init machinery
        self.emit("static INIT: Once = Once::new();")
        self.emit("thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }")
        self.emit("pub fn ensure_init() {")
        self.emit("    if INITIALIZING.with(|c| c.get()) { return; }")
        self.emit("    INIT.call_once(|| {")
        self.emit("        INITIALIZING.with(|c| c.set(true));")
        self.emit('        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT ' + self.dotted + ': {} {}", e.class_name, e.message); } });')
        self.emit("        INITIALIZING.with(|c| c.set(false));")
        self.emit("    });")
        self.emit("}")
        # accessors
        for name in sorted(mod_globals | self.decorated):
            rname = rust_ident(name)
            self.emit(f"pub fn g_{rname}() -> Py {{ ensure_init(); G_{rname}.with(|c| c.borrow().clone()) }}")
            self.emit(f"#[allow(non_snake_case)]\npub fn s_{rname}(v: Py) {{ ensure_init(); G_{rname}.with(|c| *c.borrow_mut() = v); }}")
        # module body
        self.emit("fn module_body() {")
        self.rebound_pending = set(self.rebound)
        ctx = Ctx(self, 'module', self.dotted)
        ctx.globals_all = mod_globals | fn_names
        for stmt in body:
            if isinstance(stmt, (ast.Import, ast.ImportFrom)):
                if self.import_globals:
                    if isinstance(stmt, ast.Import):
                        als = [al.asname or al.name.split('.')[0] for al in stmt.names]
                    else:
                        als = [al.asname or al.name for al in stmt.names]
                    for alias in als:
                        if alias in self.import_globals:
                            full, item = self.import_globals[alias]
                            expr = self.ref_fn_value(full, item) if item in self.fns.get(full, set()) else self.ref_global_read(full, item)
                            self.emit(f"    s_{rust_ident(alias)}({expr});")
                continue
            if isinstance(stmt, (ast.FunctionDef, ast.ClassDef)):
                if isinstance(stmt, ast.ClassDef):
                    self.gen_class_reg(stmt)
                continue
            self.gen_stmt(stmt, ctx)
        # bind decorated functions at body start... Python binds them in order; approximate:
        # bind them at the END of module body? No: top of body, before other statements may use.
        self.emit("}")
        # insert decorated bindings right after module_body opening
        binds = []
        for f in fdefs:
            if f.name in self.decorated:
                rname = rust_ident(f.name)
                inner = f"{rname}__wrapped"
                expr = f"py_func({rust_string(f.name)}, |args: &[Py]| {inner}(args))"
                for dec in reversed(f.decorator_list):
                    expr = f"py_call(&{self.gen_expr(dec, ctx)}, &[{expr}])"
                binds.append(f"    s_{rname}({expr});")
        if binds:
            for i, l in enumerate(self.lines):
                if l == "fn module_body() {":
                    self.lines[i + 1:i + 1] = binds
                    break
        # functions
        for f in fdefs:
            self.gen_fn(f, top=True)
        for c in cdefs:
            self.gen_class(c)
        self.gen_module_obj(mod_globals, fdefs, cdefs)
        self.gen_register(mod_globals, fdefs, cdefs)
        return '\n'.join(self.lines) + '\n'

    def gen_module_obj(self, mod_globals, fdefs, cdefs):
        names = sorted((mod_globals | self.decorated) | {f.name for f in fdefs} | {c.name for c in cdefs})
        plain_fns = {f.name for f in fdefs if f.name not in self.decorated}
        cls_names = {c.name for c in cdefs}
        self.emit("pub fn module_obj() -> Py {")
        self.emit("    ensure_init();")
        self.emit(f"    Py::Module(Rc::new(PyModule {{ name: {rust_string('bot.' + self.dotted)}.to_string(),")
        self.emit("        get: Rc::new(|attr: &str| match attr {")
        for n in names:
            r = rust_ident(n)
            if n in cls_names:
                self.emit(f"            {rust_string(n)} => Some(py_func({rust_string(n)}, |args: &[Py]| {r}(args))),")
            elif n in plain_fns:
                self.emit(f"            {rust_string(n)} => Some(py_func({rust_string(n)}, |args: &[Py]| {r}(args))),")
            else:
                self.emit(f"            {rust_string(n)} => Some(g_{r}()),")
        for alias, target in self.info.mod.items():
            if alias not in names:
                self.emit(f'            {rust_string(alias)} => Some({self.ref_module(target)}),')
        for alias, target in self.info.std.items():
            if alias not in names:
                self.emit(f'            {rust_string(alias)} => Some(stdlib_module({rust_string(target)})),')
        self.emit("            _ => None,")
        self.emit("        }),")
        self.emit("        set: Some(Rc::new(|attr: &str, v: Py| match attr {")
        for n in sorted(mod_globals | self.decorated):
            r = rust_ident(n)
            self.emit(f"            {rust_string(n)} => s_{r}(v),")
        self.emit("            _ => {},")
        self.emit("        })),")
        self.emit("    }))")
        self.emit("}")

    def gen_register(self, mod_globals, fdefs, cdefs):
        self.emit("pub fn register() {")
        self.emit(f"    pyrt::xrefs::reg_module({rust_string(self.dotted)}, module_obj);")
        plain_fns = {f.name for f in fdefs if f.name not in self.decorated}
        for f in fdefs:
            r = rust_ident(f.name)
            if f.name in self.decorated:
                self.emit(f"    pyrt::xrefs::reg_fn({rust_string(self.dotted)}, {rust_string(f.name)}, py_func({rust_string(f.name)}, |args: &[Py]| py_call(&g_{r}(), args)));")
            else:
                self.emit(f"    pyrt::xrefs::reg_fn({rust_string(self.dotted)}, {rust_string(f.name)}, py_func({rust_string(f.name)}, |args: &[Py]| {r}(args)));")
        for c in cdefs:
            r = rust_ident(c.name)
            self.emit(f"    pyrt::xrefs::reg_fn({rust_string(self.dotted)}, {rust_string(c.name)}, py_func({rust_string(c.name)}, |args: &[Py]| {r}(args)));")
        for n in sorted(mod_globals | self.decorated):
            r = rust_ident(n)
            self.emit(f"    pyrt::xrefs::reg_access({rust_string(self.dotted)}, {rust_string(n)}, g_{r}, s_{r});")
        self.emit("}")

    def gen_class_reg(self, node):
        for item in node.body:
            if isinstance(item, ast.FunctionDef):
                cname = rust_ident(node.name)
                mname = rust_ident(item.name)
                self.emit(f"    register_class_method({rust_string(node.name)}, {rust_string(item.name)}, py_func({rust_string(node.name + '.' + item.name)}, |args: &[Py]| {cname}__{mname}(args)));")

    # ------------------------------------------------------------ function codegen
    def gen_fn(self, node, top=False, prefix=""):
        rname = rust_ident(prefix + node.name)
        if top and node.name in self.decorated:
            rname = rname + "__wrapped" 
        params = [a.arg for a in node.args.args]
        defaults = node.args.defaults
        ndef = len(defaults)
        ctx = Ctx(self, 'fn', node.name)
        ctx.params = params
        locals_, globals_declared = self.fn_scope(node)
        ctx.locals = locals_
        ctx.global_names = globals_declared
        ctx.generator = is_generator(node)
        self.emit(f"pub fn {rname}(args: &[Py]) -> Py {{")
        self.emit("    ensure_init();")
        self.emit("    let mut __pos: Vec<Py> = Vec::new();")
        self.emit("    let mut __kw: Vec<(String, Py)> = Vec::new();")
        self.emit("    for a in args {")
        self.emit("        match a {")
        self.emit("            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }")
        self.emit("            other => __pos.push(other.clone()),")
        self.emit("        }")
        self.emit("    }")
        for i, p in enumerate(params):
            rp = rust_ident(p)
            dflt_idx = i - (len(params) - ndef)
            self.emit(f"    let mut {rp}: Py = if __pos.len() > {i} {{ __pos[{i}].clone() }}")
            self.emit(f"    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == {rust_string(p)}) {{ v.clone() }}")
            if dflt_idx >= 0:
                self.emit(f"    else {{ {self.gen_expr(defaults[dflt_idx], ctx)} }};")
            else:
                self.emit(f"    else {{ exc::raise(type_err(&format!(\"missing required argument: {p}\"))) }};")
        if node.args.vararg:
            vp = rust_ident(node.args.vararg.arg)
            self.emit(f"    let mut {vp}: Py = py_list(__pos.get({len(params)}..).unwrap_or(&[]).to_vec());")
        if node.args.kwarg:
            kp = rust_ident(node.args.kwarg.arg)
            pnames = ', '.join(rust_string(p) for p in params)
            self.emit(f"    let mut {kp}: Py = py_dict(__kw.iter().filter(|(n, _)| ![{pnames}].contains(&n.as_str())).map(|(n, v)| (py_str(n), v.clone())).collect());")
        for name in sorted(ctx.locals - set(params) - ({node.args.vararg.arg} if node.args.vararg else set()) - ({node.args.kwarg.arg} if node.args.kwarg else set())):
            self.emit(f"    let mut {rust_ident(name)}: Py = Py::None;")
        if ctx.generator:
            self.emit('    let mut __yielded: Vec<Py> = Vec::new();')
        for stmt in node.body:
            self.gen_stmt(stmt, ctx)
        self.emit("    py_list(__yielded)" if ctx.generator else "    Py::None")
        self.emit("}")

    def gen_class(self, node):
        cname = rust_ident(node.name)
        methods = [n for n in node.body if isinstance(n, ast.FunctionDef)]
        slots = []
        for n in node.body:
            if isinstance(n, ast.Assign) and any(isinstance(t, ast.Name) and t.id == '__slots__' for t in n.targets):
                if isinstance(n.value, (ast.Tuple, ast.List)):
                    slots = [e.value for e in n.value.elts if isinstance(e, ast.Constant)]
        for m in methods:
            self.gen_fn(m, top=True, prefix=node.name + "__")
        # constructor
        self.emit(f"pub fn {cname}(args: &[Py]) -> Py {{")
        self.emit("    ensure_init();")
        self.emit(f"    let inst = Py::Instance(Rc::new(PyInstance {{ class_name: {rust_string(node.name)}.to_string(), attrs: Rc::new(RefCell::new(IndexMap::new())) }}));")
        self.emit(f"    let mut full = vec![inst.clone()]; full.extend_from_slice(args);")
        if any(m.name == "__init__" for m in methods):
            self.emit(f"    {cname}____init__(&full);")
        self.emit("    inst")
        self.emit("}")

    # ------------------------------------------------------------ statements
    def gen_stmt(self, node, ctx):
        E = self.emit
        if isinstance(node, ast.Expr):
            if isinstance(node.value, ast.Yield):
                E(f'    __yielded.push({self.gen_expr(node.value.value, ctx)});')
            elif isinstance(node.value, ast.YieldFrom):
                E(f'    __yielded.extend(py_iter_or_raise(&{self.gen_expr(node.value.value, ctx)}));')
            else:
                E(f"    let _ = {self.gen_expr(node.value, ctx)};")
        elif isinstance(node, ast.Pass):
            pass
        elif isinstance(node, ast.Return):
            if getattr(ctx, 'generator', False):
                E('    return py_list(__yielded);')
            elif getattr(ctx, 'try_closure', 0):
                v = self.gen_expr(node.value, ctx) if node.value is not None else 'Py::None'
                E(f"    return py_tuple(vec![Py::Bool(true), {v}]);")
            elif node.value is None:
                E("    return Py::None;")
            else:
                E(f"    return {self.gen_expr(node.value, ctx)};")
        elif isinstance(node, ast.Assign):
            val = self.gen_expr(node.value, ctx)
            for t in node.targets:
                self.gen_assign(t, val, ctx)
            if ctx.kind == 'module':
                for t in node.targets:
                    for nm in target_names(t):
                        self.rebound_pending.discard(nm)
        elif isinstance(node, ast.AnnAssign):
            if node.value is not None:
                self.gen_assign(node.target, self.gen_expr(node.value, ctx), ctx)
                if ctx.kind == 'module' and isinstance(node.target, ast.Name):
                    self.rebound_pending.discard(node.target.id)
        elif isinstance(node, ast.AugAssign):
            op = self.binop_fn(node.op)
            cur = self.gen_expr(node.target, ctx)
            val = self.gen_expr(node.value, ctx)
            tmp = ctx.tmpname('aug')
            E(f"    let {tmp} = {op}(&{cur}, &{val});")
            self.gen_assign(node.target, tmp, ctx)
        elif isinstance(node, ast.If):
            E(f"    if py_truthy(&{self.gen_expr(node.test, ctx)}) {{")
            for s in node.body:
                self.gen_stmt(s, ctx)
            if node.orelse:
                E("    } else {")
                for s in node.orelse:
                    self.gen_stmt(s, ctx)
            E("    }")
        elif isinstance(node, ast.While):
            ctx.loop_depth = getattr(ctx, 'loop_depth', 0) + 1
            E(f"    while py_truthy(&{self.gen_expr(node.test, ctx)}) {{")
            for s in node.body:
                self.gen_stmt(s, ctx)
            E("    }")
            ctx.loop_depth -= 1
            if node.orelse:
                for s in node.orelse:
                    self.gen_stmt(s, ctx)
        elif isinstance(node, ast.For):
            ctx.loop_depth = getattr(ctx, 'loop_depth', 0) + 1
            it = ctx.tmpname('iter')
            E(f"    let {it} = py_iter_or_raise(&{self.gen_expr(node.iter, ctx)});")
            broke = ctx.tmpname('else') if node.orelse else None
            if broke:
                E(f"    let mut {broke} = false;")
            E(f"    for __item in {it} {{")
            if broke:
                pass
            self.gen_assign_target_inline(node.target, '__item', ctx)
            for s in node.body:
                self.gen_stmt(s, ctx)
            E("    }")
            ctx.loop_depth -= 1
            if node.orelse:
                for s in node.orelse:
                    self.gen_stmt(s, ctx)
        elif isinstance(node, ast.Break):
            E("    break;")
        elif isinstance(node, ast.Continue):
            if getattr(ctx, 'try_stack', []) and getattr(ctx, 'loop_depth', 0) == ctx.try_stack[-1]['depth']:
                E('    return py_tuple(vec![Py::Int(2), Py::None]);')
                ctx.try_stack[-1]['continue'] = True
            else:
                E("    continue;")
        elif isinstance(node, ast.Try):
            body_tmp = ctx.tmpname('try')
            ret_v, val_v, unh_v, hnd_v = (ctx.tmpname('ret'), ctx.tmpname('val'),
                                          ctx.tmpname('unhandled'), ctx.tmpname('handled'))
            E(f"    let {body_tmp} = exc::catch(|| {{")
            ctx.try_closure = getattr(ctx, 'try_closure', 0) + 1
            ctx.try_stack = getattr(ctx, 'try_stack', [])
            ctx.try_stack.append({'depth': getattr(ctx, 'loop_depth', 0), 'continue': False})
            for s in node.body:
                self.gen_stmt(s, ctx)
            try_info = ctx.try_stack.pop()
            ctx.try_closure -= 1
            E("        py_tuple(vec![Py::Bool(false), Py::None])")
            E("    });")
            E(f"    let mut {ret_v}: Py = Py::Bool(false);")
            E(f"    let mut {val_v}: Py = Py::None;")
            E(f"    let mut {unh_v}: Option<exc::PyExc> = None;")
            E(f"    match {body_tmp} {{")
            E(f"        Ok(__t) => {{ let __p = py_iter_or_raise(&__t); {ret_v} = __p[0].clone(); {val_v} = __p[1].clone(); }}")
            E("        Err(__e) => {")
            E(f"            let mut {hnd_v} = false;")
            for h in node.handlers:
                names = self.handler_names(h)
                cond = "true" if names is None else f"exc::matches_any(&__e, &[{', '.join(rust_string(n) for n in names)}])"
                E(f"            if !{hnd_v} && {cond} {{")
                E(f"                {hnd_v} = true;")
                if h.name:
                    E(f"                let mut {rust_ident(h.name)} = exc::exc_to_py(&__e);")
                    if h.name not in ctx.locals and h.name not in ctx.params:
                        ctx.locals.add(h.name)
                self._exc_stack = getattr(self, '_exc_stack', []) + ['__e']
                for s in h.body:
                    self.gen_stmt(s, ctx)
                self._exc_stack = self._exc_stack[:-1]
                E("            }")
            E(f"            if !{hnd_v} {{ {unh_v} = Some(__e); }}")
            E("        }")
            E("    }")
            for s in node.finalbody:
                self.gen_stmt(s, ctx)
            E(f"    if let Some(__e) = {unh_v} {{ exc::raise(__e); }}")
            if ctx.kind == 'fn':
                if try_info['continue']:
                    if ctx.try_stack and ctx.try_stack[-1]['depth'] == getattr(ctx, 'loop_depth', 0):
                        E(f"    if py_eq(&{ret_v}, &Py::Int(2)) {{ return py_tuple(vec![{ret_v}, {val_v}]); }}")
                        ctx.try_stack[-1]['continue'] = True
                    else:
                        E(f"    if py_eq(&{ret_v}, &Py::Int(2)) {{ continue; }}")
                if getattr(ctx, 'try_closure', 0):
                    E(f"    if py_truthy(&{ret_v}) {{ return py_tuple(vec![{ret_v}, {val_v}]); }}")
                else:
                    E(f"    if py_truthy(&{ret_v}) {{ return {val_v}; }}")
        elif isinstance(node, ast.Raise):
            if node.exc is None:
                stack = getattr(self, '_exc_stack', [])
                if stack:
                    E(f"    exc::raise({stack[-1]}.clone());")
                else:
                    E("    exc::raise(exc::PyExc::new(\"RuntimeError\", \"re-raise\"));")
            else:
                e = self.gen_expr(node.exc, ctx)
                E(f"    match {e} {{")
                E("        Py::Exc(x) => exc::raise((*x).clone()),")
                E(f"        other => exc::raise(exc::PyExc::new(\"RuntimeError\", &py_str_of(&other))),")
                E("    }")
        elif isinstance(node, ast.Delete):
            for t in node.targets:
                if isinstance(t, ast.Subscript):
                    E(f"    let _ = py_delitem(&{self.gen_expr(t.value, ctx)}, &{self.gen_expr(t.slice, ctx)}).unwrap_or(());")
                elif isinstance(t, ast.Name):
                    self.gen_assign(t, 'Py::None', ctx)
        elif isinstance(node, ast.Global):
            pass
        elif isinstance(node, ast.Nonlocal):
            pass
        elif isinstance(node, ast.Assert):
            E(f"    if !py_truthy(&{self.gen_expr(node.test, ctx)}) {{ exc::raise(exc::PyExc::new(\"AssertionError\", \"\")); }}")
        elif isinstance(node, (ast.Import, ast.ImportFrom)):
            if ctx.kind != 'module':
                self.gen_local_import(node, ctx)
            else:
                aliases = [a.asname or (a.name.split('.')[0] if isinstance(node, ast.Import) else a.name) for a in node.names]
                for alias in aliases:
                    if alias in self.import_globals:
                        full, item = self.import_globals[alias]
                        expr = self.ref_fn_value(full, item) if item in self.fns.get(full,set()) else self.ref_global_read(full,item)
                        E(f'    s_{rust_ident(alias)}({expr});')

        elif isinstance(node, ast.FunctionDef):
            # nested function: bind a closure capturing current locals
            self.gen_nested_fn(node, ctx)
        elif isinstance(node, ast.ClassDef):
            self.warn("nested class skipped", node)
        elif isinstance(node, ast.With):
            self.warn("with-statement approximated", node)
            for s in node.body:
                self.gen_stmt(s, ctx)
        else:
            self.warn(f"unhandled stmt {type(node).__name__}", node)

    def handler_names(self, h):
        if h.type is None:
            return None
        names = []
        nodes = h.type.elts if isinstance(h.type, ast.Tuple) else [h.type]
        for n in nodes:
            if isinstance(n, ast.Name):
                names.append(n.id)
            elif isinstance(n, ast.Attribute):
                names.append(n.attr)
        return names

    def gen_assign(self, target, val, ctx):
        E = self.emit
        if isinstance(target, ast.Name):
            name = target.id
            if ctx.kind == 'module':
                E(f'    if std::env::var_os("PYBOT_TRACE_INIT").is_some() {{ eprintln!("SET {self.dotted}.{name}"); }}')
            if name in ctx.global_names:
                E(f"    s_{rust_ident(name)}({val});")
            elif ctx.kind == 'module':
                E(f"    s_{rust_ident(name)}({val});")
            elif name in ctx.locals or name in ctx.params:
                E(f"    {rust_ident(name)} = {val};")
            else:
                # comprehension-bound or new: declare
                E(f"    let mut {rust_ident(name)}: Py = {val};")
                ctx.locals.add(name)
        elif isinstance(target, ast.Subscript):
            if isinstance(target.slice, ast.Slice):
                self.warn("slice assignment approximated", target)
                E(f"    let _ = py_setitem(&{self.gen_expr(target.value, ctx)}, &Py::Int(0), {val});")
            else:
                E(f"    let _ = py_setitem(&{self.gen_expr(target.value, ctx)}, &{self.gen_expr(target.slice, ctx)}, {val});")
        elif isinstance(target, ast.Attribute):
            E(f"    let _ = py_setattr(&{self.gen_expr(target.value, ctx)}, {rust_string(target.attr)}, {val});")
        elif isinstance(target, (ast.Tuple, ast.List)):
            names = target_names(target)
            starred = None
            for i, e in enumerate(target.elts):
                if isinstance(e, ast.Starred):
                    starred = i
            tmp = ctx.tmpname('unpk')
            E(f"    let {tmp} = py_unpack(&{val}, {len(target.elts)}, {'Some(' + str(starred) + ')' if starred is not None else 'None'});")
            for i, e in enumerate(target.elts):
                if isinstance(e, ast.Starred):
                    idx = i
                else:
                    idx = i
                self.gen_assign(e, f"{tmp}[{i}].clone()", ctx)
        else:
            self.warn(f"unhandled assign target {type(target).__name__}", target)

    def gen_assign_target_inline(self, target, item_expr, ctx):
        """for-loop target from an already-named value expr"""
        self.gen_assign(target, item_expr + ".clone()", ctx)

    def gen_nested_fn(self, node, ctx):
        rname = rust_ident(node.name)
        captures = sorted(ctx.locals | set(ctx.params))
        cap_args = ', '.join(rust_ident(c) + ".clone()" for c in captures)
        cap_names = ', '.join(rust_ident(c) for c in captures)
        # closure calling a generated top-level fn with captures prepended
        hidden = f"__nested_{node.name}_{ctx.tmp}"
        ctx.tmp += 1
        params = [a.arg for a in node.args.args]
        defaults = node.args.defaults
        ndef = len(defaults)
        locals_, globals_declared = self.fn_scope(node)
        inner_ctx = Ctx(self, 'fn', node.name)
        inner_ctx.params = captures + params
        inner_ctx.locals = locals_
        inner_ctx.global_names = globals_declared
        inner_ctx.generator = is_generator(node)
        self.emit(f"pub fn {hidden}(args: &[Py]) -> Py {{")
        self.emit("    ensure_init();")
        self.emit("    let mut __pos: Vec<Py> = args.to_vec();")
        self.emit("    let mut __kw: Vec<(String, Py)> = Vec::new();")
        self.emit("    let mut __split = __pos.len();")
        self.emit("    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }")
        self.emit("    let __pos = &args[..__split];")
        for i, p in enumerate(captures):
            self.emit(f"    let mut {rust_ident(p)}: Py = __pos.get({i}).cloned().unwrap_or(Py::None);")
        for i, p in enumerate(params):
            j = i + len(captures)
            dflt_idx = i - (len(params) - ndef)
            if dflt_idx >= 0:
                dflt = self.gen_expr(defaults[dflt_idx], inner_ctx)
                self.emit(f"    let mut {rust_ident(p)}: Py = __pos.get({j}).cloned().or_else(|| __kw.iter().find(|(n, _)| n == {rust_string(p)}).map(|(_, v)| v.clone())).unwrap_or_else(|| {dflt});")
            else:
                self.emit(f"    let mut {rust_ident(p)}: Py = __pos.get({j}).cloned().or_else(|| __kw.iter().find(|(n, _)| n == {rust_string(p)}).map(|(_, v)| v.clone())).unwrap_or(Py::None);")
        if node.args.vararg:
            vp = rust_ident(node.args.vararg.arg)
            off = len(captures) + len(params)
            self.emit(f"    let mut {vp}: Py = py_list(__pos.get({off}..).unwrap_or(&[]).to_vec());")
        if node.args.kwarg:
            kp = rust_ident(node.args.kwarg.arg)
            pnames = ', '.join(rust_string(p) for p in params)
            self.emit(f"    let mut {kp}: Py = py_dict(__kw.iter().filter(|(n, _)| ![{pnames}].contains(&n.as_str())).map(|(n, v)| (py_str(n), v.clone())).collect());")
        for name in sorted(locals_ - set(captures) - set(params) - ({node.args.vararg.arg} if node.args.vararg else set()) - ({node.args.kwarg.arg} if node.args.kwarg else set())):
            self.emit(f"    let mut {rust_ident(name)}: Py = Py::None;")
        if inner_ctx.generator:
            self.emit('    let mut __yielded: Vec<Py> = Vec::new();')
        for s in node.body:
            self.gen_stmt(s, inner_ctx)
        self.emit("    py_list(__yielded)" if inner_ctx.generator else "    Py::None")
        self.emit("}")
        E = self.emit
        if ctx.kind == 'module':
            shadow = ' '.join(f'let {rust_ident(c)} = {rust_ident(c)}.clone();' for c in captures)
            close = f"py_func({rust_string(node.name)}, move |args: &[Py]| {{ let mut full = vec![{cap_args}]; full.extend_from_slice(args); {hidden}(&full) }})"
            if captures:
                close = '{{ ' + shadow + ' ' + close + ' }}'
            E(f"    s_{rname}({close});")
        else:
            shadow = ' '.join(f'let {rust_ident(c)} = {rust_ident(c)}.clone();' for c in captures)
            close = f"py_func({rust_string(node.name)}, move |args: &[Py]| {{ let mut full = vec![{cap_args}]; full.extend_from_slice(args); {hidden}(&full) }})"
            if captures:
                close = '{{ ' + shadow + ' ' + close + ' }}'
            E(f"    {rname} = {close};")

    # ------------------------------------------------------------ expressions
    def binop_fn(self, op):
        return {
            ast.Add: 'op_add', ast.Sub: 'op_sub', ast.Mult: 'op_mul', ast.Div: 'op_truediv',
            ast.FloorDiv: 'op_floordiv', ast.Mod: 'op_modulo', ast.Pow: 'op_pow',
            ast.BitAnd: 'op_bitand', ast.BitOr: 'op_bitor', ast.BitXor: 'op_bitxor',
            ast.LShift: 'op_lshift', ast.RShift: 'op_rshift', ast.MatMult: 'op_mul',
        }[type(op)]

    def gen_expr(self, node, ctx):
        if ctx.kind == 'module' and isinstance(node, (ast.List, ast.Tuple, ast.Set, ast.Dict)) and not getattr(node, '_outlined', False):
            size = len(node.keys) if isinstance(node, ast.Dict) else len(node.elts)
            if size > 12:
                node._outlined = True
                name = ctx.tmpname('data')
                expr = self.gen_expr(node, ctx)
                self.emit(f'    fn {name}() -> Py {{ {expr} }}')
                return f'{name}()'
        if node is None:
            return 'Py::None'
        if isinstance(node, ast.Constant):
            v = node.value
            if v is None:
                return 'Py::None'
            if v is True:
                return 'Py::Bool(true)'
            if v is False:
                return 'Py::Bool(false)'
            if isinstance(v, int):
                return f'Py::Int({v})'
            if isinstance(v, float):
                return f'Py::Float({v!r})'
            if isinstance(v, str):
                return f'py_str({rust_string(v)})'
            if isinstance(v, bytes):
                return 'Py::Bytes(Rc::new(vec![' + ', '.join(str(b) for b in v) + ']))'
            if v is Ellipsis:
                return 'Py::Ellipsis'
            return 'Py::None'
        if isinstance(node, ast.Name):
            return self.resolve_name(node.id, ctx)
        if isinstance(node, ast.Attribute):
            return self.gen_attribute(node, ctx)
        if isinstance(node, ast.Call):
            return self.gen_call(node, ctx)
        if isinstance(node, ast.BinOp):
            fn = self.binop_fn(node.op)
            return f'{fn}(&{self.gen_expr(node.left, ctx)}, &{self.gen_expr(node.right, ctx)})'
        if isinstance(node, ast.UnaryOp):
            v = self.gen_expr(node.operand, ctx)
            if isinstance(node.op, ast.Not):
                return f'Py::Bool(!py_truthy(&{v}))'
            if isinstance(node.op, ast.USub):
                return f'op_neg(&{v})'
            if isinstance(node.op, ast.UAdd):
                return v
            if isinstance(node.op, ast.Invert):
                return f'op_invert(&{v})'
        if isinstance(node, ast.BoolOp):
            parts = [self.gen_expr(v, ctx) for v in node.values]
            if isinstance(node.op, ast.Or):
                expr = parts[-1]
                for p in reversed(parts[:-1]):
                    expr = f'{{ let __b = {p}; if py_truthy(&__b) {{ __b }} else {{ {expr} }} }}'
                return expr
            else:
                expr = parts[-1]
                for p in reversed(parts[:-1]):
                    expr = f'{{ let __b = {p}; if !py_truthy(&__b) {{ __b }} else {{ {expr} }} }}'
                return expr
        if isinstance(node, ast.Compare):
            return self.gen_compare(node, ctx)
        if isinstance(node, ast.IfExp):
            return f'if py_truthy(&{self.gen_expr(node.test, ctx)}) {{ {self.gen_expr(node.body, ctx)} }} else {{ {self.gen_expr(node.orelse, ctx)} }}'
        if isinstance(node, ast.Subscript):
            if isinstance(node.slice, ast.Slice):
                lo = self.gen_expr(node.slice.lower, ctx)
                hi = self.gen_expr(node.slice.upper, ctx)
                st = self.gen_expr(node.slice.step, ctx)
                return f'slice_or_raise(&{self.gen_expr(node.value, ctx)}, &{lo}, &{hi}, &{st})'
            return f'geti(&{self.gen_expr(node.value, ctx)}, &{self.gen_expr(node.slice, ctx)})'
        if isinstance(node, (ast.List, ast.Tuple, ast.Set)):
            builder = {ast.List: 'py_list', ast.Tuple: 'py_tuple', ast.Set: 'py_set'}[type(node)]
            if any(isinstance(e, ast.Starred) for e in node.elts):
                parts = []
                for e in node.elts:
                    if isinstance(e, ast.Starred):
                        parts.append('__v.extend(py_iter_or_raise(&' + self.gen_expr(e.value, ctx) + '));')
                    else:
                        parts.append('__v.push(' + self.gen_expr(e, ctx) + ');')
                return '{ let mut __v: Vec<Py> = Vec::new(); ' + ' '.join(parts) + ' ' + builder + '(__v) }'
            return builder + '(vec![' + ', '.join(self.gen_expr(e, ctx) for e in node.elts) + '])'
        if isinstance(node, ast.Dict):
            if any(k is None for k in node.keys):
                parts = []
                for k, v in zip(node.keys, node.values):
                    if k is None:
                        parts.append(self.gen_expr(v, ctx))
                    else:
                        parts.append('py_dict(vec![(' + self.gen_expr(k, ctx) + ', ' + self.gen_expr(v, ctx) + ')])')
                return 'dict_spread(&[' + ', '.join(parts) + '])'
            return 'py_dict(vec![' + ', '.join('(' + self.gen_expr(k, ctx) + ', ' + self.gen_expr(v, ctx) + ')' for k,v in zip(node.keys,node.values)) + '])'
        if isinstance(node, ast.Starred):
            return self.gen_expr(node.value, ctx)
        if isinstance(node, ast.JoinedStr):
            pushes = []
            for v in node.values:
                if isinstance(v, ast.Constant):
                    pushes.append(f'__s.push_str({rust_string(v.value)});')
                elif isinstance(v, ast.FormattedValue):
                    conv = v.conversion
                    spec = ''
                    if v.format_spec is not None:
                        spec = self.fspec_literal(v.format_spec, ctx)
                        inner = self.gen_expr(v.value, ctx)
                        pushes.append(f'__s.push_str(&fstr(&{inner}, {conv}, &{spec}));')
                    else:
                        inner = self.gen_expr(v.value, ctx)
                        pushes.append(f'__s.push_str(&fstr(&{inner}, {conv}, ""));')
            return '{ let mut __s = String::new(); ' + ' '.join(pushes) + ' py_string(__s) }'
        if isinstance(node, ast.FormattedValue):
            return f'py_string(fstr(&{self.gen_expr(node.value, ctx)}, {node.conversion}, ""))'
        if isinstance(node, (ast.ListComp, ast.SetComp, ast.GeneratorExp, ast.DictComp)):
            return self.gen_comp(node, ctx)
        if isinstance(node, ast.Lambda):
            return self.gen_lambda(node, ctx)
        if isinstance(node, ast.Await):
            return self.gen_expr(node.value, ctx)
        if isinstance(node, ast.Yield):
            self.warn("yield outside generator support", node)
            return 'Py::None'
        if isinstance(node, ast.NamedExpr):
            val = self.gen_expr(node.value, ctx)
            name = rust_ident(node.target.id)
            return f'{{ {name} = {val}; {name}.clone() }}'
        self.warn(f"unhandled expr {type(node).__name__}", node)
        return 'Py::None'

    def fspec_literal(self, spec, ctx):
        parts = []
        for v in spec.values:
            if isinstance(v, ast.Constant):
                parts.append(v.value)
            elif isinstance(v, ast.FormattedValue):
                parts.append('{}')  # dynamic spec: rare; approximate
        return rust_string(''.join(parts))

    def gen_compare(self, node, ctx):
        ops = []
        for op, comp in zip(node.ops, node.comparators):
            ops.append((op, comp))
        left = self.gen_expr(node.left, ctx)
        if len(ops) == 1:
            op, comp = ops[0]
            right = self.gen_expr(comp, ctx)
            return self.compare_expr(left, op, right)
        # chained: a < b < c  ->  { let t1 = b; if a < t1 { t1 < c } else { false } }
        tmp = ctx.tmpname('cmp')
        n = len(ops)
        operands = [left] + [self.gen_expr(comp, ctx) for _o, comp in ops]
        result = self.compare_expr(f'{tmp}{n - 1}', ops[-1][0], operands[n])
        for k in reversed(range(1, n)):
            prev = f'{tmp}{k - 1}' if k > 1 else operands[0]
            cond = self.compare_expr(prev, ops[k - 1][0], f'{tmp}{k}')
            result = f'{{ let {tmp}{k}: Py = {operands[k]}; if py_truthy(&{cond}) {{ {result} }} else {{ Py::Bool(false) }} }}'
        return result

    def compare_expr(self, left, op, right):
        if isinstance(op, ast.Eq):
            return f'Py::Bool(py_eq(&{left}, &{right}))'
        if isinstance(op, ast.NotEq):
            return f'Py::Bool(!py_eq(&{left}, &{right}))'
        if isinstance(op, ast.Lt):
            return f'op_lt(&{left}, &{right})'
        if isinstance(op, ast.LtE):
            return f'op_le(&{left}, &{right})'
        if isinstance(op, ast.Gt):
            return f'op_gt(&{left}, &{right})'
        if isinstance(op, ast.GtE):
            return f'op_ge(&{left}, &{right})'
        if isinstance(op, ast.In):
            return f'Py::Bool(py_contains(&{right}, &{left}))'
        if isinstance(op, ast.NotIn):
            return f'Py::Bool(!py_contains(&{right}, &{left}))'
        if isinstance(op, ast.Is):
            return f'Py::Bool(py_is(&{left}, &{right}))'
        if isinstance(op, ast.IsNot):
            return f'Py::Bool(!py_is(&{left}, &{right}))'
        return 'Py::Bool(false)'

    # ------------------------------------------------------------ name resolution
    def resolve_name(self, name, ctx):
        r = rust_ident(name)
        if name == '__name__':
            return 'py_str(' + rust_string('bot.' + self.dotted) + ')'
        if name == '__path__':
            return 'py_list(vec![' + 'py_str(' + rust_string(self.dotted) + ')])'
        if name == '__package__':
            return 'py_str(' + rust_string('bot.' + self.dotted.rsplit('.', 1)[0]) + ')'
        if name in ('True', 'False', 'None'):
            return {'True': 'Py::Bool(true)', 'False': 'Py::Bool(false)', 'None': 'Py::None'}[name]
        if ctx.kind == 'fn':
            c = ctx
            while c is not None:
                if name in c.params or name in c.locals:
                    return f'{r}.clone()'
                c = getattr(c, 'parent', None)
            if name in ctx.global_names:
                return f'g_{r}()'
            # module function?
            if name in self.fns.get(self.dotted, set()):
                if name in self.decorated:
                    return f'g_{r}()'
                return f'py_func({rust_string(name)}, |args: &[Py]| {r}(args))'
            if name in (self.glbs.get(self.dotted, set())):
                return f'g_{r}()'
        else:
            # module body context
            if name in getattr(self, 'rebound_pending', set()):
                return f'py_func({rust_string(name)}, |args: &[Py]| {r}__wrapped(args))'
            if name in self.fns.get(self.dotted, set()) and name not in self.decorated:
                return f'py_func({rust_string(name)}, |args: &[Py]| {r}(args))'
            if name in self.glbs.get(self.dotted, set()) or name in self.decorated:
                return f'g_{r}()'
        if name in getattr(self, 'import_globals', {}):
            return f'g_{r}()'
        # imports
        if name in self.info.std:
            return f'stdlib_module({rust_string(self.info.std[name])})'
        if name in self.info.func:
            tdotted, item = self.info.func[name]
            if item in self.classes.get(tdotted, set()) or item in self.fns.get(tdotted, set()):
                return self.ref_fn_value(tdotted, item)
            return self.ref_global_read(tdotted, item)
        if name in self.info.cls:
            tdotted, cname = self.info.cls[name]
            return self.ref_fn_value(tdotted, cname)
        if name in self.info.mod:
            return self.ref_module(self.info.mod[name])
        if name in self.info.unknown:
            modname, item = self.info.unknown[name]
            return f'stdlib_attr({rust_string(modname)}, {rust_string(item)}).unwrap_or(Py::None)'
        # builtins as values
        if name in BUILTINS:
            return f'py_func({rust_string(name)}, |args: &[Py]| call_builtin({rust_string(name)}, args).unwrap_or_else(|e| exc::raise(e)))'
        # unknown -> try module global (might be assigned later), else builtin error
        if name in self.glbs.get(self.dotted, set()):
            return f'g_{r}()'
        return f'unbound_name({rust_string(name)})'

    def gen_attribute(self, node, ctx):
        # module.attr resolution first
        base = self.attr_base_module(node.value, ctx)
        if base is not None:
            kind, data = base
            if kind == 'stdlib':
                modname = data
                if isinstance(node.value, ast.Attribute):
                    # e.g. datetime.date.today handled by caller path building
                    pass
                return f'stdlib_attr({rust_string(modname)}, {rust_string(node.attr)}).unwrap_or_else(|e| exc::raise(e))'
            if kind == 'botmod':
                dotted = data
                item = node.attr
                if item in self.classes.get(dotted, set()) or item in self.fns.get(dotted, set()):
                    return self.ref_fn_value(dotted, item)
                sub = dotted + '.' + item if dotted else item
                if sub in self.mods:
                    return self.ref_module(sub)
                return self.ref_global_read(dotted, item)
        # datetime.date / datetime.datetime / datetime.timedelta attribute chains
        chain = self.attr_chain(node)
        if chain and chain[0] in self.info.std and self.info.std[chain[0]] == 'datetime' and len(chain) == 2:
            return f'stdlib_attr("datetime", {rust_string(chain[1])}).unwrap_or_else(|e| exc::raise(e))'
        return f'gattr(&{self.gen_expr(node.value, ctx)}, {rust_string(node.attr)})'

    def attr_chain(self, node):
        parts = []
        cur = node
        while isinstance(cur, ast.Attribute):
            parts.append(cur.attr)
            cur = cur.value
        if isinstance(cur, ast.Name):
            parts.append(cur.id)
            return list(reversed(parts))
        return None

    def attr_base_module(self, node, ctx=None):
        if isinstance(node, ast.Name):
            name = node.id
            if ctx is not None and ctx.kind == 'fn' and (name in ctx.locals or name in ctx.params):
                return None
            if name in self.info.std:
                return ('stdlib', self.info.std[name])
            if name in self.info.mod:
                return ('botmod', self.info.mod[name])
            if name in self.info.func:
                return None
        return None

    def gen_local_import(self, node, ctx):
        """Function-level import: bind each alias to a local Py value."""
        info = ImportInfo()
        resolve_one_import(node, info, self.dotted, self.mods, self.classes)
        E = self.emit
        def bind(alias, expr):
            r = rust_ident(alias)
            E(f"    {r} = {expr};" if alias in ctx.locals else f"    let mut {r}: Py = {expr};")
            ctx.locals.add(alias)
        for alias, modname in info.std.items():
            bind(alias, f'stdlib_module({rust_string(modname)})')
        for alias, tdotted in info.mod.items():
            bind(alias, self.ref_module(tdotted))
        for alias, (tdotted, item) in info.cls.items():
            bind(alias, self.ref_fn_value(tdotted, item))
        for alias, (tdotted, item) in info.func.items():
            if item in self.classes.get(tdotted, set()) or item in self.fns.get(tdotted, set()):
                bind(alias, self.ref_fn_value(tdotted, item))
            else:
                bind(alias, self.ref_global_read(tdotted, item))
        for alias, (modname, item) in info.unknown.items():
            bind(alias, f'stdlib_attr({rust_string(modname)}, {rust_string(item)}).unwrap_or(Py::None)')

    # ------------------------------------------------------------ calls
    def gen_call(self, node, ctx):
        args, kwargs = [], []
        has_star = False
        for a in node.args:
            if isinstance(a, ast.Starred):
                has_star = True
                args.append(('star', self.gen_expr(a.value, ctx)))
            else:
                args.append(('pos', self.gen_expr(a, ctx)))
        for k in node.keywords:
            if k.arg is None:
                kwargs.append(('starstar', self.gen_expr(k.value, ctx)))
            else:
                kwargs.append((k.arg, self.gen_expr(k.value, ctx)))

        # attribute call on module?  mod.fn(...)
        if isinstance(node.func, ast.Attribute):
            chain = self.attr_chain(node.func)
            if chain and chain[0] in self.info.std and not (ctx.kind == 'fn' and chain[0] in ctx.locals):
                modname = self.info.std[chain[0]]
                path = '.'.join(chain[1:])
                return self.call_with_args(f'|args: &[Py]| stdlib_call({rust_string(modname)}, {rust_string(path)}, args).unwrap_or_else(|e| exc::raise(e))',
                                           args, kwargs, has_star, ctx, node)
            base = self.attr_base_module(node.func.value, ctx)
            if base is not None and base[0] == 'botmod':
                dotted = base[1]
                item = node.func.attr
                if item in self.fns.get(dotted, set()) or item in self.classes.get(dotted, set()):
                    return self.call_with_args(self.ref_builder(dotted, item),
                                               args, kwargs, has_star, ctx, node)
                sub = dotted + '.' + item if dotted else item
                if sub in self.mods:
                    self.warn(f"call to module {sub}?", node)
                return self.call_with_args(f'|args: &[Py]| py_call(&{self.ref_global_read(dotted, item)}, args)',
                                           args, kwargs, has_star, ctx, node)
            # method call
            recv = self.gen_expr(node.func.value, ctx)
            return self.call_with_args(f'|args: &[Py]| mcall(&{recv}, {rust_string(node.func.attr)}, args)',
                                       args, kwargs, has_star, ctx, node)
        if isinstance(node.func, ast.Name):
            name = node.func.id
            # Function-local bindings shadow imports and builtins.
            if ctx.kind == 'fn' and (name in ctx.locals or name in ctx.params):
                return self.call_with_args(f'|args: &[Py]| py_call(&{rust_ident(name)}, args)', args, kwargs, has_star, ctx, node)
            # direct resolutions
            if name in ('True', 'False', 'None'):
                return 'Py::None'
            if name in BUILTINS:
                return self.call_with_args(f'|args: &[Py]| call_builtin({rust_string(name)}, args).unwrap_or_else(|e| exc::raise(e))',
                                           args, kwargs, has_star, ctx, node)
            if name in self.info.std:
                return self.call_with_args(f'|args: &[Py]| stdlib_call({rust_string(self.info.std[name])}, {rust_string(name)}, args).unwrap_or_else(|e| exc::raise(e))',
                                           args, kwargs, has_star, ctx, node)
            if name in self.info.func:
                tdotted, item = self.info.func[name]
                if item in self.classes.get(tdotted, set()) or item in self.fns.get(tdotted, set()):
                    return self.call_with_args(self.ref_builder(tdotted, item),
                                               args, kwargs, has_star, ctx, node)
                return self.call_with_args(f'|args: &[Py]| py_call(&{self.ref_global_read(tdotted, item)}, args)',
                                           args, kwargs, has_star, ctx, node)
            if name in self.info.cls:
                tdotted, cname = self.info.cls[name]
                return self.call_with_args(self.ref_builder(tdotted, cname),
                                           args, kwargs, has_star, ctx, node)
            if name in self.info.unknown:
                modname, item = self.info.unknown[name]
                return self.call_with_args(f'|args: &[Py]| stdlib_call({rust_string(modname)}, {rust_string(item)}, args).unwrap_or_else(|e| exc::raise(e))',
                                           args, kwargs, has_star, ctx, node)
            if ctx.kind == 'fn':
                c = ctx
                is_local = False
                while c is not None:
                    if name in c.locals or name in c.params:
                        is_local = True
                        break
                    c = getattr(c, 'parent', None)
                if is_local:
                    return self.call_with_args(f'|args: &[Py]| py_call(&{rust_ident(name)}, args)',
                                               args, kwargs, has_star, ctx, node)
            if name in self.fns.get(self.dotted, set()):
                if name in self.decorated:
                    return self.call_with_args(f'|args: &[Py]| py_call(&g_{rust_ident(name)}(), args)',
                                               args, kwargs, has_star, ctx, node)
                return self.call_with_args(f'|args: &[Py]| {rust_ident(name)}(args)',
                                           args, kwargs, has_star, ctx, node)
            if name in self.glbs.get(self.dotted, set()):
                return self.call_with_args(f'|args: &[Py]| py_call(&g_{rust_ident(name)}(), args)',
                                           args, kwargs, has_star, ctx, node)
            # exception constructor by name
            if name.endswith('Error') or name.endswith('Exception'):
                return self.call_with_args(f'|args: &[Py]| call_builtin({rust_string(name)}, args).unwrap_or_else(|e| exc::raise(e))',
                                           args, kwargs, has_star, ctx, node)
            self.warn(f"unresolved call {name}", node)
            return f'unbound_name({rust_string(name)})'
        # call of arbitrary expression
        fexpr = self.gen_expr(node.func, ctx)
        return self.call_with_args(f'|args: &[Py]| py_call(&{fexpr}, args)',
                                   args, kwargs, has_star, ctx, node)

    def decorated_set(self, dotted):
        # decorated functions known in that module (approximation: track per module in fns scan)
        if not hasattr(self, '_dec_cache'):
            self._dec_cache = {}
        if dotted not in self._dec_cache:
            decorated = set()
            path = self.mods.get(dotted)
            if path:
                try:
                    tree = ast.parse(open(path).read())
                    decorated = {n.name for n in tree.body if isinstance(n, ast.FunctionDef) and n.decorator_list}
                except Exception:
                    pass
            self._dec_cache[dotted] = decorated
        return self._dec_cache[dotted]

    def call_with_args(self, fn_builder, args, kwargs, has_star, ctx, node):
        if not has_star and not kwargs:
            values = ', '.join(v for _, v in args)
            prefix = '|args: &[Py]| '
            if fn_builder.startswith(prefix):
                core = fn_builder[len(prefix):]
                # Only replace the last argument-slice reference.
                at = core.rfind('args')
                if at >= 0:
                    core = core[:at] + '&[' + values + ']' + core[at+4:]
                    return core
            return '(' + fn_builder + ')(&[' + values + '])'
        pieces = []
        for kind, val in args:
            if kind == 'star':
                pieces.append(f'__args.extend(py_iter_or_raise(&{val}));')
            else:
                pieces.append(f'__args.push({val});')
        kw_entries = []
        for kind, val in kwargs:
            if kind == 'starstar':
                pieces.append(f'for (k, v) in kw_pairs(&{val}) {{ __kws.push((k, v)); }}')
            else:
                kw_entries.append(f'({rust_string(kind)}.to_string(), {val})')
        head = '{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); '
        mid = ' '.join(pieces)
        if kw_entries:
            mid += f' __kws.extend(vec![{", ".join(kw_entries)}]);'
        mid += ' if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } '
        tail = f'({fn_builder})(&__args) }}'
        return head + mid + tail

    # ------------------------------------------------------------ comprehensions / lambdas
    def gen_comp(self, node, ctx):
        """[... for ...] etc. as an immediately-invoked closure."""
        is_dict = isinstance(node, ast.DictComp)
        is_set = isinstance(node, ast.SetComp)
        lines = ['{ (|| -> Py {']
        if is_dict:
            lines.append('let mut __out: Vec<(Py, Py)> = Vec::new();')
        else:
            lines.append('let mut __out: Vec<Py> = Vec::new();')
        inner = Ctx(self, 'fn', 'comp')
        inner.params = []
        inner.locals = set()
        inner.global_names = set()
        inner.parent = ctx
        if is_dict:
            inner.comp_body = ('dict', (node.key, node.value))
        else:
            inner.comp_body = ('elt', node.elt)
        self.comp_loop(node.generators, 0, lines, inner)
        if is_dict:
            lines.append('py_dict(__out) })() }')
        elif is_set:
            lines.append('py_set(__out) })() }')
        else:
            lines.append('py_list(__out) })() }')
        return ' '.join(lines)

    def comp_loop(self, generators, i, lines, ctx):
        if i >= len(generators):
            node = getattr(ctx, 'node', None)
            return
        gen = generators[i]
        iter_expr = self.gen_expr_scoped(gen.iter, ctx)
        target = gen.target
        lines.append(f'for __item in py_iter_or_raise(&{iter_expr}) {{')
        self.comp_bind(target, '__item.clone()', lines, ctx)
        for cond in gen.ifs:
            lines.append(f'if py_truthy(&{self.gen_expr_scoped(cond, ctx)}) {{')
            lines.append('')
        # recurse: emit rest inline
        if i + 1 < len(generators):
            self.comp_loop(generators, i + 1, lines, ctx)
        else:
            body = getattr(ctx, 'comp_body', None)
            if body:
                kind, elt = body
                if kind == 'dict':
                    k, v = elt
                    lines.append(f'__out.push(({self.gen_expr_scoped(k, ctx)}, {self.gen_expr_scoped(v, ctx)}));')
                else:
                    lines.append(f'__out.push({self.gen_expr_scoped(elt, ctx)});')
        for _ in gen.ifs:
            lines.append('}')
        lines.append('}')

    def comp_bind(self, target, expr, lines, ctx):
        for name in target_names(target):
            ctx.locals.add(name)
        if isinstance(target, ast.Name):
            lines.append(f'let mut {rust_ident(target.id)}: Py = {expr};')
        elif isinstance(target, (ast.Tuple, ast.List)):
            tmp = ctx.tmpname('unpk')
            star = next((i for i,e in enumerate(target.elts) if isinstance(e, ast.Starred)), None)
            star_expr = 'None' if star is None else f'Some({star})'
            lines.append(f'let {tmp} = py_unpack(&{expr}, {len(target.elts)}, {star_expr});')
            for i, e in enumerate(target.elts):
                if isinstance(e, ast.Name):
                    lines.append(f'let mut {rust_ident(e.id)}: Py = {tmp}[{i}].clone();')
                    ctx.locals.add(e.id)
                else:
                    self.comp_bind(e, f'{tmp}[{i}].clone()', lines, ctx)

    def gen_expr_scoped(self, node, ctx):
        """Expression inside a comprehension: locals of comp shadow, then outer."""
        return self.gen_expr(node, ctx)

    def gen_lambda(self, node, ctx):
        params = [a.arg for a in node.args.args]
        defaults = node.args.defaults
        ndef = len(defaults)
        captures = sorted((ctx.locals | set(ctx.params)))
        cap = ', '.join(rust_ident(c) + '.clone()' for c in captures)
        prologue = []
        for i, p in enumerate(params):
            dflt_idx = i - (len(params) - ndef)
            if dflt_idx >= 0:
                dflt = self.gen_expr(defaults[dflt_idx], ctx)
                prologue.append(f'let {rust_ident(p)}: Py = __kwp(args, {i}, {rust_string(p)}).unwrap_or_else(|| {dflt});')
            else:
                prologue.append(f'let {rust_ident(p)}: Py = __kwp(args, {i}, {rust_string(p)}).unwrap_or(Py::None);')
        inner = Ctx(self, 'fn', 'lambda')
        if node.args.vararg:
            vp = node.args.vararg.arg
            prologue.append(f'let {rust_ident(vp)} = py_tuple(args.get({len(params)}..).unwrap_or(&[]).to_vec());')
            params.append(vp)
        inner.params = params
        inner.locals = set()
        inner.global_names = set()
        inner.parent = ctx
        body = self.gen_expr(node.body, inner)
        caps_decl = ' '.join(f'let {rust_ident(c)} = {rust_ident(c)}.clone();' for c in captures)
        lam = ('py_func("lambda", move |args: &[Py]| { ' + caps_decl + ' '
               + ' '.join(prologue) + ' ' + body + ' })')
        if captures:
            shadow = ' '.join(f'let {rust_ident(c)} = {rust_ident(c)}.clone();' for c in captures)
            lam = '{ ' + shadow + ' ' + lam + ' }'
        return lam


def modpath_to_dotted(gen, modpath):
    # crate::gen::knowledge::cooking -> knowledge.cooking
    parts = modpath.split('::')[2:]
    parts = [p[:-1] if p.endswith('_') and p[:-1] in RUST_KEYWORDS else p for p in parts]
    return '.'.join(parts)

BUILTINS = set("""len str int float bool list tuple set frozenset dict isinstance type any all enumerate range
reversed sorted min max sum abs round zip map filter ord chr hex oct bin pow divmod hash id getattr
hasattr setattr callable format print next iter object Exception ValueError TypeError KeyError
IndexError AttributeError RuntimeError StopIteration ZeroDivisionError ImportError SyntaxError
NameError LookupError ArithmeticError OverflowError OSError FileNotFoundError UnicodeDecodeError
NotImplementedError RecursionError vars globals locals open input repr super delattr exit quit bytes bytearray memoryview slice complex BaseException LookupError UnicodeError UnicodeEncodeError EOFError KeyboardInterrupt SystemExit FloatingPointError""".split())

# ---------------------------------------------------------------- driver
CRATE_PKGS = {  # crate -> top-level packages it holds, in dependency order
    'gen_understanding': ['understanding'],
    'gen_memory': ['memory'],
    'gen_knowledge': ['knowledge'],
    'gen_compose': ['compose'],
    'gen_modules': ['modules'],
    'gen_core': ['runtime', '__main__'],
}
CRATE_DEPS = {
    'gen_understanding': [],
    'gen_memory': ['gen_understanding'],
    'gen_knowledge': ['gen_memory', 'gen_understanding'],
    'gen_compose': ['gen_knowledge', 'gen_memory', 'gen_understanding'],
    'gen_modules': ['gen_compose', 'gen_knowledge', 'gen_memory', 'gen_understanding'],
    'gen_core': ['gen_modules', 'gen_compose', 'gen_knowledge', 'gen_memory', 'gen_understanding'],
}

def package_prefixes(mods):
    pkgs = set()
    for dotted in mods:
        parts = dotted.split('.')
        for i in range(1, len(parts)):
            pkgs.add('.'.join(parts[:i]))
    return pkgs

def out_path_for(dotted, pkgs):
    parts = [rust_ident(p) for p in dotted.split('.')]
    crate = crate_of(dotted)
    base = os.path.join(CRATES, crate, 'src')
    if dotted in pkgs:
        return os.path.join(base, *parts, 'mod.rs')
    if len(parts) == 1:
        return os.path.join(base, parts[0] + '.rs')
    return os.path.join(base, *parts[:-1], parts[-1] + '.rs')

CRATE_MANIFEST = """[package]
name = "{crate}"
version = "0.1.0"
edition = "2021"

[dependencies]
{deps}
indexmap = "2"
"""

PYRT_MANIFEST = """[package]
name = "pyrt"
version = "0.1.0"
edition = "2021"

[dependencies]
regex = "1"
fancy-regex = "0.14"
indexmap = "2"
unicode-normalization = "0.1"
"""

META_MANIFEST = """[package]
name = "gen_meta"
version = "0.1.0"
edition = "2021"

[dependencies]
{deps}
"""

def write_crate_files(mods, fns, glbs, classes):
    pkgs = package_prefixes(mods)
    crate_mods = {}
    for dotted in mods:
        crate_mods.setdefault(crate_of(dotted), []).append(dotted)
    for crate, members in sorted(crate_mods.items()):
        cdir = os.path.join(CRATES, crate)
        os.makedirs(os.path.join(cdir, 'src'), exist_ok=True)
        open(os.path.join(cdir, 'Cargo.toml'), 'w').write(CRATE_MANIFEST.format(crate=crate, deps='pyrt = { path = "../pyrt" }'))
        decls = {}
        for dotted in members:
            parts = dotted.split('.')
            for i in range(len(parts)):
                decls.setdefault('.'.join(parts[:i]), set()).add(parts[i])
        for parent, kids in decls.items():
            if parent:
                path = os.path.join(cdir, 'src', *[rust_ident(x) for x in parent.split('.')], 'mod.rs')
                existing = open(path).read() if os.path.exists(path) else ''
                existing = re.sub(r'^pub mod \w+;\n', '', existing, flags=re.M)
                childdecls = ''.join('pub mod ' + rust_ident(k) + ';\n' for k in sorted(kids))
                os.makedirs(os.path.dirname(path), exist_ok=True)
                # Inner attributes must precede child module declarations.
                lines = existing.splitlines(True)
                i = 0
                while i < len(lines) and (lines[i].startswith('#![') or not lines[i].strip()): i += 1
                open(path, 'w').write(''.join(lines[:i]) + childdecls + ''.join(lines[i:]))
            else:
                rootdecls = ''.join('pub mod ' + rust_ident(k) + ';\n' for k in sorted(kids))
        rootdecls += 'pub fn register_all() {\n'
        for dotted in sorted(members):
            rootdecls += 'crate::' + '::'.join(rust_ident(x) for x in dotted.split('.')) + '::register();\n'
        rootdecls += '}\n'
        open(os.path.join(cdir, 'src', 'lib.rs'), 'w').write(rootdecls)
    # pyrt crate manifest (sources moved by the shell script)
    os.makedirs(os.path.join(CRATES, 'pyrt'), exist_ok=True)
    open(os.path.join(CRATES, 'pyrt', 'Cargo.toml'), 'w').write(PYRT_MANIFEST)
    # gen_meta crate
    mdir = os.path.join(CRATES, 'gen_meta')
    os.makedirs(os.path.join(mdir, 'src'), exist_ok=True)
    all_gen = sorted(crate_mods)
    deps = ['pyrt = { path = "../pyrt" }'] + [f'{d} = {{ path = "../{d}" }}' for d in all_gen]
    open(os.path.join(mdir, 'Cargo.toml'), 'w').write(META_MANIFEST.format(deps='\n'.join(deps)))
    lines = ["//! Registry of bot/modules entries, arbiter and cross-crate registration.",
             "#![allow(dead_code)]",
             "use pyrt::value::*;",
             "use pyrt::module_entry::ModuleEntry;",
             "",
             "pub fn register_all() {"]
    for crate in all_gen:
        lines.append(f"    {crate}::register_all();")
    lines.append("}")
    lines.append("")
    mod_children = sorted(d.split('.')[1] for d in mods if d.startswith('modules.') and d.count('.') == 1)
    lines.append("pub fn registry() -> Vec<ModuleEntry> {")
    lines.append("    vec![")
    for stem in mod_children:
        dotted = f"modules.{stem}"
        path = rust_path(dotted)
        pri = f"|| match {path}::g_PRIORITY() {{ Py::Int(i) => i, Py::Float(f) => f as i64, _ => 0 }}" if 'PRIORITY' in glbs.get(dotted, set()) else "|| 0"
        obs = f'Some(|args| pyrt::xrefs::xcall("{dotted}", "observe", args))' if 'observe' in fns.get(dotted, set()) else "None"
        resp = f'Some(|args| pyrt::xrefs::xcall("{dotted}", "respond", args))' if 'respond' in fns.get(dotted, set()) else "None"
        aft = f'Some(|args| pyrt::xrefs::xcall("{dotted}", "after", args))' if 'after' in fns.get(dotted, set()) else "None"
        lines.append(f"        ModuleEntry {{ name: {rust_string(stem)}, priority: {pri}, observe: {obs}, respond: {resp}, after: {aft} }},")
    lines.append("    ]")
    lines.append("}")
    lines.append("")
    if 'modules.arbiter' in mods and 'choose' in fns.get('modules.arbiter', set()):
        lines.append('pub fn arbiter_choose() -> Option<fn(&[Py]) -> Py> { Some(' + rust_path('modules.arbiter') + '::choose) }')
    else:
        lines.append("pub fn arbiter_choose() -> Option<fn(&[Py]) -> Py> { None }")
    open(os.path.join(mdir, 'src', 'lib.rs'), 'w').write('\n'.join(lines) + '\n')

def write_changed(path, text):
    if os.path.exists(path) and open(path).read() == text: return
    open(path, 'w').write(text)

def main():
    only = [a for a in sys.argv[1:] if not a.startswith('-')]
    mods = discover()
    classes = find_classes(mods)
    fns, glbs = find_defs(mods)
    pkgs = package_prefixes(mods)
    all_warnings = []
    failed = []
    for dotted in sorted(mods):
        if only and dotted not in only:
            continue
        try:
            gen = Gen(dotted, mods[dotted], mods, fns, glbs, classes)
            out = gen.generate()
            path = out_path_for(dotted, pkgs)
            os.makedirs(os.path.dirname(path), exist_ok=True)
            write_changed(path, out)
            all_warnings.extend(gen.warnings)
        except Exception as e:
            failed.append((dotted, repr(e)))
    if not only:
        write_crate_files(mods, fns, glbs, classes)
    for d, e in failed:
        print(f"FAILED {d}: {e}", file=sys.stderr)
    for w in all_warnings:
        print("WARN", w, file=sys.stderr)
    print(f"generated {len(mods) - len(failed) if not only else len(only) - len(failed)} modules, {len(failed)} failed, {len(all_warnings)} warnings")

if __name__ == '__main__':
    main()
