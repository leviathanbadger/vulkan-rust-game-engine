mod euler1;
mod euler2;
mod euler3;
mod euler4;
mod time;

fn main() {
    time!(euler1::euler1());
    time!(euler2::euler2());
    time!(euler3::euler3());
    time!(euler4::euler4());
}
