namespace EVC.IO;

internal class Command
{
    public string Name { get; init; } = string.Empty;
    public IEnumerable<(string flag, string? value)> Parameters { get; set; } = Enumerable.Empty<(string, string?)>();
}