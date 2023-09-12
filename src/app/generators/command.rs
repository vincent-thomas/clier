use super::Generator;

pub struct CommandGenerator;

impl Generator for CommandGenerator {
    fn generate(self) -> Result<(), ()> {
        let path = self::CommandGenerator::find_path();
        Ok(())
    }
}
