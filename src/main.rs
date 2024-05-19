fn main() {

	let args: Vec<String> = std::env::args().collect();

	if args.len() != 2{
		return;
	}

	let size: &usize = &args[1].parse::<usize>().unwrap();

    let mut array: Vec<u32> = (0..*size).map(|_| rand::random::<u32>()).collect();
	let end: usize = array.len();
	
	quicksort(&mut array[0..end]);

	//	dbg!(&array);
}

fn quicksort(a: &mut [u32]){

	let lst: usize = a.len();

	if a.len() <= 1{
		return;
	}

	//todo fix
	let m = partition(a);

	//[a..b] => slice [a bis b-1]
	quicksort(&mut a[0..m]);
	quicksort(&mut a[m+1..lst]);

}

fn partition(slice: &mut [u32]) -> usize{
	
	let mut i: usize = 0;
	let mut j: usize = 0;
	let last: usize = slice.len()-1;

	let p: u32 = slice[last];

	while j < slice.len()-1{
	
		if slice[j] <= p {
			slice.swap(i, j);
			i+=1;
		}

		j+=1;
	}
	slice.swap(i, last);

	i
}
 
