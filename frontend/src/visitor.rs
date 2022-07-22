use crate::ast::*;

pub trait ExprVisitor {
    type Ident1;
    type Extra1;
    type Ident2;
    type Extra2;
    
    fn visit_ident(&mut self, ident: Self::Ident1) -> Self::Ident2;
    fn visit_extra(&mut self, ext: Self::Extra1) -> Self::Extra2;
    

    fn visit_expr(
        &mut self,
        expr: Expr<Self::Ident1,Self::Extra1>
    ) -> Expr<Self::Ident2, Self::Extra2> {
        self.walk_expr(expr)
    }

    fn walk_expr(
        &mut self,
        expr: Expr<Self::Ident1,Self::Extra1>
    ) -> Expr<Self::Ident2, Self::Extra2> {
        match expr {
            Expr::Lit(expr) => Expr::Lit(self.visit_expr_lit(expr)),
            Expr::Prim(expr) => Expr::Prim(self.visit_expr_prim(expr)),
            Expr::Var(expr) => Expr::Var(self.visit_expr_var(expr)),
            Expr::Lam(expr) => Expr::Lam(self.visit_expr_lam(expr)),
            Expr::App(expr) => Expr::App(self.visit_expr_app(expr)),
            Expr::Chain(expr) => Expr::Chain(self.visit_expr_chain(expr)),
            Expr::Let(expr) => Expr::Let(self.visit_expr_let(expr)),
            Expr::Case(expr) => Expr::Case(self.visit_expr_case(expr)),
        }
    }

    fn visit_expr_lit(&mut self, expr: ExprLit<Self::Extra1>) -> ExprLit<Self::Extra2> {
        self.walk_expr_lit(expr)
    }

    fn walk_expr_lit(&mut self, expr: ExprLit<Self::Extra1>) -> ExprLit<Self::Extra2> {
        let ExprLit { lit, span: ext } = expr;
        let ext = self.visit_extra(ext);
        ExprLit { lit, span: ext }
    }

    fn visit_expr_prim(&mut self, expr: ExprPrim<Self::Extra1>) -> ExprPrim<Self::Extra2> {
        self.walk_expr_prim(expr)
    }

    fn walk_expr_prim(&mut self, expr: ExprPrim<Self::Extra1>) -> ExprPrim<Self::Extra2> {
        let ExprPrim { prim, span: ext } = expr;
        let ext = self.visit_extra(ext);
        ExprPrim { prim, span: ext }
    }

    fn visit_expr_var(&mut self,
        expr: ExprVar<Self::Ident1,Self::Extra1>,
    ) -> ExprVar<Self::Ident2,Self::Extra2> {
        self.walk_expr_var(expr)
    }

    fn walk_expr_var(
        &mut self,
        expr: ExprVar<Self::Ident1,Self::Extra1>,
    ) -> ExprVar<Self::Ident2,Self::Extra2> {
        let ExprVar { name, span: ext } = expr;
        let name = self.visit_ident(name);
        let ext = self.visit_extra(ext);
        ExprVar { name, span: ext }
    }

    fn visit_expr_lam(
        &mut self,
        expr: ExprLam<Self::Ident1,Self::Extra1>,
    ) -> ExprLam<Self::Ident2,Self::Extra2> {
        self.walk_expr_lam(expr)
    }

    fn walk_expr_lam(
        &mut self,
        expr: ExprLam<Self::Ident1,Self::Extra1>,
    ) -> ExprLam<Self::Ident2,Self::Extra2> {
        let ExprLam { args, body, span: ext } = expr;
        let args = args.into_iter()
            .map(|arg| self.visit_ident(arg))
            .collect();
        let body = Box::new(self.visit_expr(*body));
        let ext = self.visit_extra(ext);
        ExprLam { args, body, span: ext }
    }

    fn visit_expr_app(
        &mut self,
        expr: ExprApp<Self::Ident1,Self::Extra1>,
    ) -> ExprApp<Self::Ident2,Self::Extra2> {
        self.walk_expr_app(expr)
    }

    fn walk_expr_app(
        &mut self,
        expr: ExprApp<Self::Ident1,Self::Extra1>,
    ) -> ExprApp<Self::Ident2,Self::Extra2> {
        let ExprApp { func, args, span: ext } = expr;
        let func = Box::new(self.visit_expr(*func));
        let args = args.into_iter()
            .map(|arg| self.visit_expr(arg))
            .collect();
        let ext = self.visit_extra(ext);
        ExprApp { func, args, span: ext }
    }

    fn visit_expr_chain(
        &mut self,
        expr: ExprChain<Self::Ident1,Self::Extra1>,
    ) -> ExprChain<Self::Ident2,Self::Extra2> {
        self.walk_expr_chain(expr)
    }

    fn walk_expr_chain(
        &mut self,
        expr: ExprChain<Self::Ident1,Self::Extra1>,
    ) -> ExprChain<Self::Ident2,Self::Extra2> {
        let ExprChain { head, tail, span: ext } = expr;
        let head = Box::new(self.visit_expr(*head));
        let tail = tail.into_iter()
            .map(|(op,expr)| (self.visit_ident(op),self.visit_expr(expr)))
            .collect();

        let ext = self.visit_extra(ext);
        ExprChain { head, tail, span: ext }
    }

    fn visit_expr_let(
        &mut self,
        expr: ExprLet<Self::Ident1,Self::Extra1>,
    ) -> ExprLet<Self::Ident2,Self::Extra2> {
        self.walk_expr_let(expr)
    }

    fn walk_expr_let(
        &mut self,
        expr: ExprLet<Self::Ident1,Self::Extra1>,
    ) -> ExprLet<Self::Ident2,Self::Extra2> {
        let ExprLet { decls, body, span: ext } = expr;
        let decls = decls.into_iter()
            .map(|decl| self.visit_decl(decl))
            .collect();
        let body = Box::new(self.visit_expr(*body));
        let ext = self.visit_extra(ext);
        ExprLet { decls, body, span: ext }
    }

    fn visit_expr_case(
        &mut self,
        expr: ExprCase<Self::Ident1,Self::Extra1>,
    ) -> ExprCase<Self::Ident2,Self::Extra2> {
        self.walk_expr_case(expr)
    }

    fn walk_expr_case(
        &mut self,
        expr: ExprCase<Self::Ident1,Self::Extra1>,
    ) -> ExprCase<Self::Ident2,Self::Extra2> {
        let ExprCase { expr, rules, span: ext } = expr;

        let expr = Box::new(self.visit_expr(*expr));
        let rules = rules.into_iter()
            .map(|rule| self.visit_rule(rule))
            .collect();
        let ext = self.visit_extra(ext);

        ExprCase { expr, rules, span: ext }
    }

    fn visit_rule(
        &mut self,
        rule: Rule<Self::Ident1,Self::Extra1>
    ) -> Rule<Self::Ident2,Self::Extra2> {
        let Rule { pat, body, span: ext } = rule;
        todo!()
    }
    

    fn visit_decl(
        &mut self,
        expr: Decl<Self::Ident1,Self::Extra1>,
    ) -> Decl<Self::Ident2,Self::Extra2> {
        match expr {
            Decl::Val(_) => todo!(),
            Decl::Data(_) => todo!(),
            Decl::Type(_) => todo!(),
            Decl::Opr(_) => todo!(),
        }
    }

    



    /*


    fn visit_app(&mut self, expr: CoreApp) -> Expr {
        let CoreApp { func, args } = expr;
        let func = self.visit_atom(func);
        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        Expr::App(CoreApp { func, args })
    }

    fn visit_let(&mut self, expr: CoreLet) -> Expr {
        let CoreLet { decl, cont } = expr;
        let decl = self.visit_decl(decl);
        let cont = Box::new(self.walk_cexpr(*cont));
        Expr::Let(CoreLet { decl, cont })
    }

    fn visit_fix(&mut self, expr: CoreFix) -> Expr {
        let CoreFix { decls, cont } = expr;
        let decls = decls.into_iter()
            .map(|def| self.visit_decl(def))
            .collect();
        let cont = Box::new(self.walk_cexpr(*cont));
        Expr::Fix(CoreFix { decls, cont })
    }

    fn visit_opr(&mut self, expr: CoreOpr) -> Expr {
        let CoreOpr { prim, args, bind, cont } = expr;
        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        let bind = self.visit_var_def(bind);
        let cont = Box::new(self.walk_cexpr(*cont));
        Expr::Opr(CoreOpr { prim, cont, args, bind })
    }

    fn visit_case(&mut self, expr: CoreCase) -> Expr {
        let CoreCase { arg, brs } = expr;
        let arg = self.visit_atom(arg);
        let brs = brs.into_iter()
            .map(|br| self.walk_cexpr(br))
            .collect();
        Expr::Case(CoreCase { arg, brs })
    }

    fn visit_rec(&mut self, expr: CoreRec) -> Expr {
        let CoreRec { size, bind, cont } = expr;
        let bind = self.visit_var_def(bind);
        let cont = Box::new(self.walk_cexpr(*cont));
        Expr::Rec(CoreRec { size, bind, cont })
    }

    fn visit_set(&mut self, expr: CoreSet) -> Expr {
        let CoreSet { rec, idx, arg, cont } = expr;
        let rec = self.visit_atom(rec);
        let arg = self.visit_atom(arg);
        let cont = Box::new(self.walk_cexpr(*cont));
        Expr::Set(CoreSet { rec, idx, arg, cont })
    }

    fn visit_get(&mut self, expr: CoreGet) -> Expr {
        let CoreGet { rec, idx, bind, cont } = expr;
        let rec = self.visit_atom(rec);
        let bind = self.visit_var_def(bind);
        let cont = Box::new(self.walk_cexpr(*cont));
        Expr::Get(CoreGet { rec, idx, bind, cont })
    }

    fn visit_halt(&mut self, arg: Atom) -> Expr {
        Expr::Halt(self.visit_atom(arg))
    }

    fn visit_tag(&mut self, tag: Tag, cont: Box<Expr>) -> Expr {
        Expr::Tag(tag, Box::new(self.walk_cexpr(*cont)))
    }
}

pub trait VisitorDownTop {
    fn walk_cexpr(&mut self, expr: Expr) -> Expr {
        match expr {
            Expr::App(expr) => self.visit_app(expr),
            Expr::Let(expr) => self.visit_let(expr),
            Expr::Fix(expr) => self.visit_fix(expr),
            Expr::Opr(expr) => self.visit_opr(expr),
            Expr::Case(expr) => self.visit_case(expr),
            Expr::Rec(expr) => self.visit_rec(expr),
            Expr::Set(expr) => self.visit_set(expr),
            Expr::Get(expr) => self.visit_get(expr),
            Expr::Halt(arg) => self.visit_halt(arg),
            Expr::Tag(tag, cont) => self.visit_tag(tag, cont),
        }
    }
    fn visit_var_def(&mut self, sym: Symbol) -> Symbol {
        sym
    }

    fn visit_var_use(&mut self, sym: Symbol) -> Symbol {
        sym
    }

    fn visit_atom(&mut self, atom: Atom) -> Atom {
        if let Atom::Var(var) = atom {
            Atom::Var(self.visit_var_use(var))
        } else {
            atom
        }
    }

    fn visit_decl(&mut self, decl: CoreDecl) -> CoreDecl {
        let CoreDecl { func, args, body } = decl;
        let body = Box::new(self.walk_cexpr(*body));
        let func = self.visit_var_def(func);
        let args = args.iter()
            .map(|arg| self.visit_var_def(*arg))
            .collect();
        CoreDecl { func, args, body }
    }

    fn visit_app(&mut self, expr: CoreApp) -> Expr {
        let CoreApp { func, args } = expr;
        let func = self.visit_atom(func);
        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        Expr::App(CoreApp { func, args })
    }

    fn visit_let(&mut self, expr: CoreLet) -> Expr {
        let CoreLet { decl, cont } = expr;
        let cont = Box::new(self.walk_cexpr(*cont));
        let decl = self.visit_decl(decl);
        Expr::Let(CoreLet { decl, cont })
    }

    fn visit_fix(&mut self, expr: CoreFix) -> Expr {
        let CoreFix { decls, cont } = expr;
        let cont = Box::new(self.walk_cexpr(*cont));
        let decls = decls.into_iter()
            .map(|def| self.visit_decl(def))
            .collect();
        Expr::Fix(CoreFix { decls, cont })
    }

    fn visit_opr(&mut self, expr: CoreOpr) -> Expr {
        let CoreOpr { prim, args, bind, cont } = expr;
        let cont = Box::new(self.walk_cexpr(*cont));
        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        let bind = self.visit_var_def(bind);
        Expr::Opr(CoreOpr { prim, cont, args, bind })
    }

    fn visit_case(&mut self, expr: CoreCase) -> Expr {
        let CoreCase { arg, brs } = expr;
        let brs = brs.into_iter()
            .map(|br| self.walk_cexpr(br))
            .collect();
        let arg = self.visit_atom(arg);
        Expr::Case(CoreCase { arg, brs })
    }

    fn visit_rec(&mut self, expr: CoreRec) -> Expr {
        let CoreRec { size, bind, cont } = expr;
        let cont = Box::new(self.walk_cexpr(*cont));
        let bind = self.visit_var_def(bind);
        Expr::Rec(CoreRec { size, bind, cont })
    }

    fn visit_set(&mut self, expr: CoreSet) -> Expr {
        let CoreSet { rec, idx, arg, cont } = expr;
        let cont = Box::new(self.walk_cexpr(*cont));
        let rec = self.visit_atom(rec);
        let arg = self.visit_atom(arg);
        Expr::Set(CoreSet { rec, idx, arg, cont })
    }

    fn visit_get(&mut self, expr: CoreGet) -> Expr {
        let CoreGet { rec, idx, bind, cont } = expr;
        let cont = Box::new(self.walk_cexpr(*cont));
        let rec = self.visit_atom(rec);
        let bind = self.visit_var_def(bind);
        Expr::Get(CoreGet { rec, idx, bind, cont })
    }

    fn visit_halt(&mut self, arg: Atom) -> Expr {
        Expr::Halt(self.visit_atom(arg))
    }

    fn visit_tag(&mut self, tag: Tag, cont: Box<Expr>) -> Expr {
        Expr::Tag(tag, Box::new(self.walk_cexpr(*cont)))
    }
    */
}
