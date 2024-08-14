use mempool_space::blocking;

fn main() {
    {
        let _res = blocking(&format!("/blocks/tip/hash"));
    }
}
