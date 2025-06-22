use criterion::{Criterion, criterion_group, criterion_main};
use leetcode_rs::_0125_valid_palindrome::{
    is_palindrome_compare_all, is_palindrome_compare_half, is_palindrome_two_pointer,
};

fn bench_palindrome(c: &mut Criterion) {
    c.bench_function("is_palindrome compare half", |b| {
        b.iter(|| is_palindrome_compare_half("A man, a plan, a canal: Panama".to_string()))
    });
}

fn bench_is_palindrome_compare_all(c: &mut Criterion) {
    c.bench_function("is_palindrome compare all", |b| {
        b.iter(|| is_palindrome_compare_all("A man, a plan, a canal: Panama".to_string()))
    });
}

fn bench_is_palindrome_two_pointers(c: &mut Criterion) {
    c.bench_function("is_palindrome two pointers", |b| {
        b.iter(|| is_palindrome_two_pointer("A man, a plan, a canal: Panama".to_string()))
    });
}
criterion_group!(
    benches,
    bench_palindrome,
    bench_is_palindrome_compare_all,
    bench_is_palindrome_two_pointers
);
criterion_main!(benches);
