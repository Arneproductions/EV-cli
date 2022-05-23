namespace EVC.IO;

internal interface ICommandService
{
    Command GetCommand(string[] args);
}