use rand;

fn main() {

    let mut array = [0; 0xFFFF].map(|_| rand::random::<i32>());
	let end: usize = array.len();

	quicksort(&mut array[0..end]);

	//	dbg!(&array);

	for (idx, int) in array.into_iter().enumerate(){

		if idx+1  == end{
			println!("Array sorted!");
			return;
		}

		if array[idx+1] < int{
			println!("{} smaller then {}", array[idx+1], int);
		}
	}
}

fn quicksort(a: &mut [i32]){

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

fn partition(slice: &mut [i32]) -> usize{
	
	let mut i: usize = 0;
	let mut j: usize = 0;
	let last: usize = slice.len()-1;

	let p: i32 = slice[last];

	while j < slice.len()-1{
	
		if slice[j] <= p {
			slice.swap(i, j);
			i+=1;
		}

		j+=1;
	}
	slice.swap(i, last);

	return i;
}
 
