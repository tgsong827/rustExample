use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    // 어떤 타입을 쓸지 명확하게 명시한다.
    generic::<char>(SGen('a'));

    // 어떤 타입을 쓸지 명확하게 명시하지 않는다.
    generic(SGen('c'));
}

struct A;           // Concrete type `A`.
struct S(A);        // Concrete type `S`.
struct SGen<T>(T);  // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}