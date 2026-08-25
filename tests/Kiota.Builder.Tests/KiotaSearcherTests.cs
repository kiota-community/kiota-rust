using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Threading;
using System.Threading.Tasks;
using Kiota.Builder.Configuration;
using Kiota.Builder.SearchProviders.GitHub;
using Kiota.Builder.Tests.Helpers;
using Microsoft.Extensions.Logging;
using Microsoft.Kiota.Abstractions;
using Microsoft.Kiota.Abstractions.Authentication;
using Moq;
using Xunit;

namespace Kiota.Builder.Tests;

public sealed class KiotaSearcherTests : IDisposable
{
    private readonly HttpClient httpClient = new();
    [Fact]
    public async Task DefensiveProgramingAsync()
    {
        Assert.Throws<ArgumentNullException>(() => new KiotaSearcher(null, new SearchConfiguration(), httpClient, null, null));
        Assert.Throws<ArgumentNullException>(() => new KiotaSearcher(new Mock<ILogger<KiotaSearcher>>().Object, null, httpClient, null, null));
        Assert.Throws<ArgumentNullException>(() => new KiotaSearcher(new Mock<ILogger<KiotaSearcher>>().Object, new SearchConfiguration(), null, null, null));
        Assert.Throws<ArgumentNullException>(() => new GitHubSearchProvider(httpClient, new Mock<ILogger<KiotaSearcher>>().Object, false, null, null, null));
        Assert.Throws<ArgumentNullException>(() => new GitHubSearchProvider(httpClient, null, false, new GitHubConfiguration(), null, null));
        Assert.Throws<ArgumentNullException>(() => new GitHubSearchProvider(null, new Mock<ILogger<KiotaSearcher>>().Object, false, new GitHubConfiguration(), null, null));
        await Assert.ThrowsAsync<ArgumentNullException>(() => new GitHubSearchProvider(httpClient, new Mock<ILogger<KiotaSearcher>>().Object, false, new GitHubConfiguration(), null, null).SearchAsync(null, null, CancellationToken.None));
    }
    private static SearchConfiguration searchConfigurationFactory => new()
    {
        GitHub = new()
        {
        }
    };
    private static KiotaSearcher CreateSearcher(HttpClient httpClient)
    {
        var authenticationProvider = GetGitHubAuthenticationProvider();
        return new KiotaSearcher(
            new Mock<ILogger<KiotaSearcher>>().Object,
            searchConfigurationFactory,
            httpClient,
            authenticationProvider,
            _ => Task.FromResult(authenticationProvider is not null));
    }
    private static IAuthenticationProvider GetGitHubAuthenticationProvider()
    {
        var token = Environment.GetEnvironmentVariable("GITHUB_TOKEN");
        if (string.IsNullOrWhiteSpace(token))
            token = Environment.GetEnvironmentVariable("GH_TOKEN");
        return string.IsNullOrWhiteSpace(token) ? null : new GitHubActionsAuthenticationProvider(token);
    }
    [RetryFact]
    public async Task GetsMicrosoftGraphBothVersionsAsync()
    {
        var searcher = CreateSearcher(httpClient);
        var results = await searcher.SearchAsync("github::microsoftgraph/msgraph-metadata", string.Empty, new CancellationToken());
        Assert.Equal(2, results.Count);
    }
    [RetryFact]
    public async Task GetsMicrosoftGraphAsync()
    {
        var searcher = CreateSearcher(httpClient);
        var results = await searcher.SearchAsync("github::microsoftgraph/msgraph-metadata/graph.microsoft.com/v1.0", string.Empty, new CancellationToken());
        Assert.Single(results);
        Assert.Equal("https://raw.githubusercontent.com/microsoftgraph/msgraph-metadata/master/openapi/v1.0/openapi.yaml", results.First().Value.DescriptionUrl.ToString());
    }
    [RetryFact]
    public async Task GetsMicrosoftGraphBetaAsync()
    {
        var searcher = CreateSearcher(httpClient);
        var results = await searcher.SearchAsync("github::microsoftgraph/msgraph-metadata/graph.microsoft.com/beta", string.Empty, new CancellationToken());
        Assert.Single(results);
        Assert.Equal("https://raw.githubusercontent.com/microsoftgraph/msgraph-metadata/master/openapi/beta/openapi.yaml", results.First().Value.DescriptionUrl.ToString());
    }
    [Fact]
    public async Task DoesntFailOnEmptyTermAsync()
    {
        var searcher = CreateSearcher(httpClient);
        var results = await searcher.SearchAsync(string.Empty, string.Empty, new CancellationToken());
        Assert.Empty(results);
    }
    [Fact]
    public async Task GetsGithubFromApisGuruAsync()
    {
        var searcher = CreateSearcher(httpClient);
        var results = await searcher.SearchAsync("github", string.Empty, new CancellationToken());
        Assert.NotEmpty(results);
    }
    [Fact]
    public async Task GetsGithubFromApisGuruWithExactMatchAsync()
    {
        var searcher = CreateSearcher(httpClient);
        var results = await searcher.SearchAsync("apisguru::github.com:api.github.com.2022-11-28", string.Empty, new CancellationToken());
        Assert.Single(results);
        var result = results.First();
        var resultUrl = result.Value.DescriptionUrl;
        var bytes = await httpClient.GetByteArrayAsync(resultUrl, cancellationToken: TestContext.Current.CancellationToken);
        Assert.NotEmpty(bytes);
    }
    public void Dispose()
    {
        httpClient.Dispose();
        GC.SuppressFinalize(this);
    }
    private sealed class GitHubActionsAuthenticationProvider(string token) : SearchProviders.GitHub.Authentication.AnonymousAuthenticationProvider
    {
        public override async Task AuthenticateRequestAsync(RequestInformation request, Dictionary<string, object> additionalAuthenticationContext = null, CancellationToken cancellationToken = default)
        {
            await base.AuthenticateRequestAsync(request, additionalAuthenticationContext, cancellationToken).ConfigureAwait(false);
            request.Headers.Add("Authorization", $"Bearer {token}");
        }
    }
}
