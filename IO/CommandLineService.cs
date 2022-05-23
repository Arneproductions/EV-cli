namespace EVC.IO;

internal class CommandLineService
{
    private readonly IDictionary<string, Command> _commands = new Dictionary<string, Command>();

    public CommandLineService(IEnumerable<Command> commands)
    {
        _commands = commands.ToDictionary(x => x.Name);
    }
}