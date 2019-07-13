use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct GenericError
{
    msg : String
}

impl GenericError
{
    pub fn new(err_msg : String) -> GenericError
    {    
        return GenericError{msg : err_msg};
    } 
}

impl fmt::Display for GenericError 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        write!(f, "{}", self.msg)
    }
}

impl error::Error for GenericError
{
    fn description(&self) -> &str 
    {
        "this is a generic error"
    }

    fn cause(&self) -> Option<&dyn error::Error> 
    {
        None
    }
}