use crate::{Expr, Format, FormatRef, IntoLabel, Label, ValueType, ViewExpr};

#[derive(Copy, Clone)]
pub struct DepFormat<const E: usize, const V: usize> {
    inner: FormatRef,
    __args: std::marker::PhantomData<[Expr; E]>,
    __views: std::marker::PhantomData<[ViewExpr; V]>,
}

fn lift<const N: usize, const M: usize>(inner: FormatRef) -> DepFormat<N, M> {
    DepFormat {
        inner,
        __args: std::marker::PhantomData,
        __views: std::marker::PhantomData,
    }
}

impl DepFormat<0, 0> {
    pub fn invoke(&self) -> Format {
        self.inner.call()
    }
}

impl<const N: usize> DepFormat<N, 0> {
    pub fn invoke_args(&self, args: [Expr; N]) -> Format {
        self.inner.call_args(args.to_vec())
    }
}

impl DepFormat<0, 1> {
    pub fn invoke_view(&self, view: ViewExpr) -> Format {
        self.inner.call_view(view)
    }
}

impl<const M: usize> DepFormat<0, M> {
    pub fn invoke_views(&self, views: [ViewExpr; M]) -> Format {
        self.inner.call_args_views(Vec::new(), views.to_vec())
    }
}

impl<const N: usize, const M: usize> DepFormat<N, M> {
    pub fn invoke_args_views(&self, args: [Expr; N], views: [ViewExpr; M]) -> Format {
        self.inner.call_args_views(args.to_vec(), views.to_vec())
    }
}

impl crate::FormatModule {
    pub fn register_format_args<const N: usize>(
        &mut self,
        name: impl IntoLabel,
        args: [(Label, ValueType); N],
        format: Format,
    ) -> DepFormat<N, 0> {
        self.register_format_args_views(name, args, [], format)
    }

    pub fn register_format_views<const M: usize>(
        &mut self,
        name: impl IntoLabel,
        views: [Label; M],
        format: Format,
    ) -> DepFormat<0, M> {
        self.register_format_args_views(name, [], views, format)
    }

    pub fn register_format_view(
        &mut self,
        name: impl IntoLabel,
        view: Label,
        format: Format,
    ) -> DepFormat<0, 1> {
        self.register_format_args_views(name, [], [view], format)
    }

    pub fn register_format_args_views<const N: usize, const M: usize>(
        &mut self,
        name: impl IntoLabel,
        args: [(Label, ValueType); N],
        views: [Label; M],
        format: Format,
    ) -> DepFormat<N, M> {
        let format_ref = self.define_format_args_views(name, args.to_vec(), views.to_vec(), format);
        lift::<N, M>(format_ref)
    }
}
