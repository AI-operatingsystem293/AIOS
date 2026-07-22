use crate::{
    context::Context,
    request::Request,
    response::Response,
};

pub trait Agent {

    fn id(&self) -> &'static str;

    fn name(&self) -> &'static str;

    fn version(&self) -> &'static str;

    fn capabilities(&self)
        -> Vec<&'static str>;

    fn execute(

        &self,

        context: &Context,

        request: Request,

    ) -> Response;
}
