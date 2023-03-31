use deku::bitvec::{BitSlice, Msb0};
use deku::{DekuError, DekuRead};

#[derive(PartialEq, Debug, DekuRead)]
pub struct Foo {
    header: [u8; 4],
    #[deku(ctx = "deku::input_bits")]
    bar: Bar,
}

#[derive(PartialEq, Debug, DekuRead)]
#[deku(ctx = "input: &'__deku_input BitSlice<u8, Msb0>")]
pub struct Bar {
    offset: u8,
    #[deku(reader = "read_at_offset(deku::rest, *offset, input)")]
    value: u8,
    other: u8,
}

fn read_at_offset<'input, 'ctx>(
    input: &'input BitSlice<u8, Msb0>,
    offset: u8,
    ctx: &'ctx BitSlice<u8, Msb0>,
) -> Result<(&'ctx BitSlice<u8, Msb0>, u8), DekuError>
where
    'input: 'ctx,
{
    let offset = offset as usize * 8;

    let (_rest, value) = u8::read(&ctx[offset..], ())?;

    Ok((input, value))
}

fn main() {
    use deku::bitvec::BitView;

    let data = &[00, 11, 22, 33, 3, 121];
    let (_rest, foo) = Foo::read(data.view_bits(), ()).unwrap();

    dbg!(foo);
}
