
struct FakePoller {
    
}

impl FakePoller {
    pub fn register(
        &mut self, 
        tap: &impl Device, 
        token: Token, 
        interest: Interest
    ) -> Result<()> {
        
        Ok(())
    }
}