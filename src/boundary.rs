use inkeeper_domain::{ProgramInfo, ProgramName};

pub trait Os {
    fn get_program_info(&self, program_name: ProgramName) -> Option<ProgramInfo>;
}
