use passive::run;

fn main() {
    pollster::block_on(run());
}
