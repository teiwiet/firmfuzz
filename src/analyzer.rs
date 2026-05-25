

const DANGER_FUNC : &[&str] = &[
    "popen",
    "system",
    "gets",
    "execve",
    "strcpy",
    "strcat",
    "sprintf",
    "vsprintf",
];

const INPUT_FUNC : &[&str] = &[
    "recv",
    "recvfrom",
    "scanf",
    "read",
    "fread",
    "getevn",
    "fgets"
];
