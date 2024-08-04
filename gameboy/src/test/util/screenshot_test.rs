pub fn compare_lcd(a: &[u8], b: &[u8]) -> bool {
	assert!(
		a.len() == b.len(),
		"Images are not of the same size, a:{}, b:{}",
		a.len(),
		b.len()
	);

	a.iter().zip(b.iter()).all(|(a, b)| a == b)
}
