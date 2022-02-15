using Microsoft.AspNetCore.Mvc;
using System.Text;

namespace sharpy.Controllers;

[ApiController]
[Route("[controller]")]
public class CacheController : ControllerBase
{
    private static readonly string[] Summaries = new[]
    {
        "Freezing", "Bracing", "Chilly", "Cool", "Mild", "Warm", "Balmy", "Hot", "Sweltering", "Scorching"
    };

    private readonly ILogger<CacheController> _logger;
    private readonly Dictionary<string, string> _cache;
        public CacheController(ILogger<CacheController> logger, Dictionary<string, string> cache)
    {
        _logger = logger;
        _cache = cache;
    }

    [HttpGet()]
    [Route("{id}")]
    public string Get(string id)
    {
        return _cache[id];
    }

    [HttpPost()]
    [Route("{id}")]
    public async Task Post(string id) {
        var rawRequestBody = await Request.GetRawBodyAsync();
        if(rawRequestBody != null) {
            _cache[id] = rawRequestBody;
        }
    }
}

public static class RequestHelpers {
    public static async Task<string> GetRawBodyAsync(this HttpRequest request, Encoding encoding = null)
    {
        if (!request.Body.CanSeek)
        {
            // We only do this if the stream isn't *already* seekable,
            // as EnableBuffering will create a new stream instance
            // each time it's called
            request.EnableBuffering();
        }

        request.Body.Position = 0;

        var reader = new StreamReader(request.Body, encoding ?? Encoding.UTF8);

        var body = await reader.ReadToEndAsync().ConfigureAwait(false);

        request.Body.Position = 0;

        return body;
    }

}
