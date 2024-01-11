#[cfg(test)]
use mockall::automock;

pub(crate) struct Module {

}

#[cfg_attr(test, automock)]
impl Module {
    pub fn new() -> Self{
        Module {}
    }

    pub(crate) fn set_module_information(&self, data: ()) -> Self {
        todo!("Load data into module")
    }
}