use crate::ui::instructions::Instruction;

pub struct Controller{


}

impl Controller {
    pub fn execute(&self, instruction: Instruction){
        match instruction{
            Instruction::Delete(_) => {}
            Instruction::Copy(_, _) => {}
            Instruction::Move(_, _) => {}
            Instruction::Color(_, _, _, _) => {}
        }
    }
}
impl Default for Controller{
    fn default() -> Self{
        Self{}
    }
}