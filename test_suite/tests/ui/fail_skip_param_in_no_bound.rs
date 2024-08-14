use info::{self as scale_info};
use scale_info::{TypeInfo, TypeInfoNoBound};

trait Config {
    type Balance;
}

struct Runtime;

impl Config for Runtime {
    type Balance = u64;
}

#[allow(unused)]
#[derive(TypeInfoNoBound)]
#[scale_info(skip_type_params(T))]
struct A<T: Config> {
    balance: T::Balance,
    marker: core::marker::PhantomData<T>,
}

fn assert_type_info<T: TypeInfo + 'static>() {}

fn main() {
    assert_type_info::<A<Runtime>>();
}
