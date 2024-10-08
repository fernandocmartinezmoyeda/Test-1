#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}

#[derive(Debug)]
enum Major {
    ComputerScience,
    ElectricalEngineering,
}

#[derive(Debug)]
struct Student {
    name: String,
    grade: GradeLevel,
    major: Major,
}

impl Student {
    fn new(name: String, grade: GradeLevel, major: Major) -> Self {
        Student {
            name,
            grade,
            major,
        }
    }

    fn introduce_yourself(&self) {
        // Use a match statement to print the grade level
        let grade = match self.grade {
            GradeLevel::Bachelor => "Bachelor",
            GradeLevel::Master => "Master",
            GradeLevel::PhD => "PhD",
        };

        // Print out the student details
        println!("{} is a {} student majoring in {:?}", self.name, grade, self.major);
    }
}

fn main() {
    let s1 = Student::new("John".to_string(), GradeLevel::Bachelor, Major::ComputerScience);
    s1.introduce_yourself();

}
