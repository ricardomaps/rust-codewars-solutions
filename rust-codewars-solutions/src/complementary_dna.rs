fn dna_strand(dna: &str) -> String {
    // Translate the DNA strand
    dna.chars()
        .map(|ch| match ch {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => panic!("not a valid dna strand"),
        })
        .collect()
}
