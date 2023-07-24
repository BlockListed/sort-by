pub const HELP: &str =
"Sort input using a specific sorting key.

Usage: sortb [OPTIONS] PATTERN [FILE]

Arguments:
	PATTERN: A regexp. The first subgroup is used by default.
		(Can be changed with -s / --subgroup)
	FILE: The file to sort, default is STDIN.

OPTIONS:
	-h, --help
		Show this page.
	-r, --reverse
		Sort in descending order instead of ascending
	-s, --subgroup
		Select which subgroup is used as a sorting key.
		0 means the whole match is used as a (integer) sorting key.
";