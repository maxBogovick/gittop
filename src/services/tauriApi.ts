import { invoke } from '@tauri-apps/api/core';

export enum TimeRange {
    Day = "DAY",
    Week = "WEEK",
    Month = "MONTH",
    Year = "YEAR"
}

export enum Metric {
    Stars = "Stars",
    Forks = "Forks",
    Updated = "Updated",
    Issues = "Issues",
    PullRequests = "Pull Requests",
    Comments = "Comments",
    Releases = "Releases"
}

export interface Repository {
    owner: string;
    name: string;
    description?: string;
    primary_language?: string;
    created_at: string;
    updated_at: string;
    stars_total: number;
    stars_delta: number;
    forks: number;
    open_issues: number;
    pull_requests_count: number;
    comments_count: number;
    releases_count: number;
    last_activity_at?: string;
    repository_url: string;
    owner_avatar_url?: string;
    license?: string;
}

export async function getTopRepositories(range: TimeRange, metric: string, language?: string, keyword?: string, page: number = 1): Promise<Repository[]> {
    return await invoke('get_top_repositories', { range, metric, language, keyword, page });
}

export async function getNewRepositories(range: TimeRange, metric?: string, language?: string, keyword?: string, page: number = 1): Promise<Repository[]> {
    return await invoke('get_new_repositories', { range, metric, language, keyword, page });
}

export async function refreshCache(): Promise<void> {
    return await invoke('refresh_cache');
}

export async function getReadme(owner: string, name: string): Promise<string> {
    return await invoke('get_repository_readme', { owner, name });
}

export async function exportRepositories(repos: Repository[], format: 'csv' | 'json'): Promise<string> {
    return await invoke('export_repositories', { repos, format });
}

export interface RedditPost {
    id: string;
    title: string;
    author: string;
    subreddit: string;
    score: number;
    num_comments: number;
    created_utc: string;
    url: string;
    selftext: string;
    permalink: string;
    thumbnail?: string;
}

export interface RedditListing {
    posts: RedditPost[];
    after?: string;
}

export async function getRedditTop(subreddit: string | undefined, timeRange: TimeRange, after?: string): Promise<RedditListing> {
    return await invoke('get_reddit_top', { subreddit, timeRange, after });
}

export async function getRedditNew(subreddit: string | undefined, after?: string): Promise<RedditListing> {
    return await invoke('get_reddit_new', { subreddit, after });
}

export async function searchReddit(query: string, timeRange: TimeRange, after?: string): Promise<RedditListing> {
    return await invoke('search_reddit', { query, timeRange, after });
}

export interface EtsyPrice {
    amount: number;
    divisor: number;
    currency_code: string;
}

export interface EtsyImage {
    url_75x75?: string;
    url_170x135?: string;
    url_570xN?: string;
    url_fullxfull?: string;
}

export interface EtsyShop {
    shop_id: number;
    shop_name: string;
}

export interface EtsyListing {
    listing_id: number;
    title: string;
    description?: string;
    url: string;
    price?: EtsyPrice;
    num_favorers?: number;
    creation_tsz?: number;
    tags?: string[];
    images?: EtsyImage[];
    shop?: EtsyShop;
}

export async function getEtsyTop(limit?: number, offset?: number): Promise<EtsyListing[]> {
    return await invoke('get_etsy_top', { limit, offset });
}

export async function getEtsyNew(limit?: number, offset?: number): Promise<EtsyListing[]> {
    return await invoke('get_etsy_new', { limit, offset });
}

export async function searchEtsy(query: string, limit?: number, offset?: number): Promise<EtsyListing[]> {
    return await invoke('search_etsy', { query, limit, offset });
}

export interface DevtoUser {
    name: string;
    username: string;
    profile_image?: string;
    twitter_username?: string;
    github_username?: string;
    website_url?: string;
}

export interface DevtoArticle {
    id: number;
    title: string;
    description?: string;
    cover_image?: string;
    social_image?: string;
    published_at: string;
    tag_list: string[];
    slug: string;
    path: string;
    url: string;
    canonical_url: string;
    comments_count: number;
    public_reactions_count: number;
    positive_reactions_count: number;
    reading_time_minutes: number;
    user: DevtoUser;
    body_html?: string;
    body_markdown?: string;
}

export async function getDevtoArticles(tag: string | null, top: number | null, state: string | null, page: number): Promise<DevtoArticle[]> {
    return await invoke('get_devto_articles', { tag, top, state, page });
}

export async function getDevtoArticleDetails(id: number): Promise<DevtoArticle> {
    return await invoke('get_devto_article_details', { id });
}

export interface StackOverflowOwner {
    display_name?: string;
    profile_image?: string;
    link?: string;
    reputation?: number;
}

export interface StackOverflowQuestion {
    question_id: number;
    title: string;
    link: string;
    score: number;
    answer_count: number;
    view_count: number;
    is_answered: boolean;
    tags: string[];
    creation_date: number;
    owner: StackOverflowOwner;
    body?: string;
}

export async function getStackOverflowQuestions(tag?: string, sort?: string, page: number = 1): Promise<StackOverflowQuestion[]> {
    return await invoke('get_stackoverflow_questions', { tag, sort, page });
}

export interface HackerNewsStory {
    id: number;
    title?: string;
    url?: string;
    by?: string;
    time: number;
    score: number;
    descendants: number;
    text?: string;
}

export async function getHackerNewsStories(storyType?: string, page: number = 1): Promise<HackerNewsStory[]> {
    return await invoke('get_hackernews_stories', { storyType, page });
}

export async function searchHackerNews(query: string, sort?: string, timeRange?: string, page: number = 1): Promise<HackerNewsStory[]> {
    return await invoke('search_hackernews', { query, sort, timeRange, page });
}

export interface MediumArticle {
    title: string;
    link: string;
    author: string;
    pub_date: string;
    categories: string[];
    content: string;
    thumbnail?: string;
}

export async function getMediumArticles(tag?: string): Promise<MediumArticle[]> {
    return await invoke('get_medium_articles', { tag });
}

export interface HashnodePost {
    id: string;
    title: string;
    brief: string;
    url: string;
    publishedAt: string;
    readTimeInMinutes: number;
    reactionCount: number;
    responseCount: number;
    author: {
        name: string;
        username: string;
        profilePicture?: string;
    };
    coverImage?: {
        url: string;
    };
}

export async function getHashnodePosts(tag?: string, feedType?: string, after?: string): Promise<HashnodePost[]> {
    return await invoke('get_hashnode_posts', { tag, feedType, after });
}

export interface ProductHuntPost {
    id: string;
    name: string;
    tagline: string;
    url: string;
    votesCount: number;
    commentsCount: number;
    thumbnail?: { url: string };
    createdAt: string;
}

export async function getProductHuntPosts(): Promise<ProductHuntPost[]> {
    return await invoke('get_producthunt_posts');
}

export interface LobstersComment {
    short_id: string;
    created_at: string;
    comment: string;
    comment_plain: string;
    depth: number;
    commenting_user: string;
    score: number;
    parent_comment: string | null;
}

export interface LobstersStory {
    short_id: string;
    title: string;
    url: string;
    score: number;
    comment_count: number;
    tags: string[];
    created_at: string;
    comments_url: string;
    submitter_user: string;
    comments?: LobstersComment[];
}

export async function getLobstersStories(storyType?: string): Promise<LobstersStory[]> {
    return await invoke('get_lobsters_stories', { storyType });
}

export async function getLobstersStoryDetails(shortId: string): Promise<LobstersStory> {
    return await invoke('get_lobsters_story_details', { shortId });
}

export interface Crate {
    id: string;
    name: string;
    description?: string;
    downloads: number;
    max_version: string;
    updated_at: string;
    homepage?: string;
    repository?: string;
}

export async function getCrates(query?: string, sort?: string, page: number = 1): Promise<Crate[]> {
    return await invoke('get_crates', { query, sort, page });
}

export interface IndieHackersPost {
    title: string;
    link: string;
    author: string;
    pub_date: string;
    content: string;
    thumbnail?: string;
}

export async function getIndieHackersPosts(): Promise<IndieHackersPost[]> {
    return await invoke('get_indiehackers_posts');
}

export interface UnifiedSearchResult {
    source: string; 
    category: string; // "Code", "Discussion", "Article", "Product"
    id: string;
    title: string;
    url: string;
    description?: string;
    author?: string;
    points?: number;
    comment_count?: number;
    created_at?: string;
    tags: string[];
}

export async function searchAll(
    query: string, 
    sources?: string[], 
    timeFilter?: string, 
    sortBy?: string,
    page?: number
): Promise<UnifiedSearchResult[]> {
    return await invoke('search_all', { 
        query, 
        sources, 
        time_filter: timeFilter,
        sort_by: sortBy,
        page
    });
}

