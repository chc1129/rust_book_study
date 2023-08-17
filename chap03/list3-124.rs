impl<'i, 't, 'a, R, P, E: 'i> ListParser<'i, 't, 'a, P>
where
    P: RuleParser<'i, Rule = R, Error = E>
        + AtRuleParser<'i, AtRule = R, Error = E>,
{
    pub fn new(input: &'a mut Parser<'i, 't>, parser: P) -> Self {
        // ...
    }
}
