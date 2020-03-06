
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R)
}

pub fn into_partition<L, R, TIn>(eithers: TIn) -> (Vec<L>, Vec<R>)
where
    TIn: IntoIterator<Item = Either<L,R>>,
{
    let mut lefts  : Vec<L> = Vec::new();
    let mut rights : Vec<R> = Vec::new();

    for i in eithers.into_iter() {
        match i {
            Either::Left(l) => lefts.push(l),
            Either::Right(r) => rights.push(r)
        }
    }
    return (lefts, rights);
}


pub fn partition<'a, L, R, TIn>(eithers: TIn) -> (Vec<&'a L>, Vec<&'a R>)
where
    TIn: Iterator<Item = &'a Either<L,R>>,
{
    let mut lefts  : Vec<&L> = Vec::new();
    let mut rights : Vec<&R> = Vec::new();

    eithers.for_each(|e| match e {
        Either::Left(ref l) => lefts.push(l),
        Either::Right(ref r) => rights.push(r)
    });

    return (lefts, rights);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn partition_moves_empty() {
        let input : Vec<Either<i32, i32>> = vec![];
        let expected : (Vec<i32>, Vec<i32>) = (vec![],vec![]);
        assert_eq!(into_partition(input), expected);
    }

    #[test]
    fn partition_moves_correctly() {
        let input : Vec<Either<i32, String>> = vec![
                Either::Right(String::from("a")),
                Either::Left(1),
                Either::Right(String::from("b")),
                Either::Left(2),
                Either::Left(3)
        ];

        let expected : (Vec<i32>, Vec<String>) = (vec![1,2,3],vec![String::from("a"),String::from("b")]);
        assert_eq!(into_partition(input), expected);
    }

    #[test]
    fn partition_refs_correctly() {
        let input : Vec<Either<i32, String>> = vec![
                Either::Right(String::from("a")),
                Either::Left(1),
                Either::Right(String::from("b")),
                Either::Left(2),
                Either::Left(3)
        ];

        let r1 = String::from("a");
        let r2 = String::from("b");
        let expected : (Vec<&i32>, Vec<&String>) = (vec![&1,&2,&3],vec![&r1, &r2]);
        assert_eq!(partition(input.iter()), expected);
    }
}
