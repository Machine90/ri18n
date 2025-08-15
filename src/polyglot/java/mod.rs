pub(crate) mod impl_4j;

pub trait IntoJava {
    fn into_java(&self) -> String;
}

pub trait IntoJavaConstruct {
    fn construct_java(&self, args: Vec<&str>) -> String;
}

pub trait IntoJavaConstructor {
    fn into_constructor(&self) -> String;
}

#[cfg(test)]
mod tests {}
