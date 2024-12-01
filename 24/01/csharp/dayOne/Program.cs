using System;

string content = await File.ReadAllTextAsync("../../01");
var lines = content.Split("\n");
var left = new List<int>();
var right = new List<int>();

foreach (var line in lines)
{
    var words = line.Split(null as char[], StringSplitOptions.RemoveEmptyEntries);
    if (words.Length == 0)
    {
        break;
    }
    // Console.WriteLine("{0}, {1}", words[0], words[1]);
    left.Add(int.Parse(words[0]));
    right.Add(int.Parse(words[1]));
}

var count = left.Select(l => right.Where(r => r == l).Count());

var products = left.Zip(count).Select((l, c) =>
{
    return (l.First * l.Second);
});
var sum = products.Sum();
Console.WriteLine(sum);
