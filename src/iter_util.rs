
pub fn sets_of_three<T: Copy>(iter: &Vec<T>) -> impl Iterator<Item=[T;3]> + '_ {
    iter.chunks_exact(3).into_iter().map(|p| [p[0], p[1], p[2]])
}
