pub const QUERY_URL: &str = "https://graphql.anilist.co";

pub fn get_query(query_name: &str) -> String {
    let search: &str = "
    query ($id: Int, $search: String, $type: MediaType) {
    Media (id: $id, search: $search, type: $type) {
            id
            season
            format
            episodes
            chapters
            volumes
            duration
            description
            status
            genres
            averageScore
            meanScore
            popularity
            siteUrl
            favourites
            bannerImage
            startDate {
                year
                month
                day
            }
            endDate {
                year
                month
                day
            }
            airingSchedule(notYetAired: true){
                nodes{
                    timeUntilAiring
                    episode
                    }
            }
            coverImage{
                extraLarge
            }
            title{
                romaji
                native
            }
        }
    }
    ";

    let user: &str = "
    query ($id: Int, $name: String) {
    User(id: $id, name: $name) {
        id
        name
        siteUrl
        updatedAt
        bannerImage
        about
        avatar {
            large
            medium
        }
        statistics {
            anime {
                count
                meanScore
                minutesWatched
                episodesWatched
                scores {
                    score
                    count
                }
                genres {
                    count
                    genre
                    meanScore
                    minutesWatched
                }
            }
            manga {
                count
                meanScore
                standardDeviation
                chaptersRead
                volumesRead
                scores {
                    score
                    count
                }
                genres {
                    count
                    genre
                    meanScore
                    minutesWatched
                }
                }
            }
        }
    }";

    let user_stats: &str = "
    query ($userId: Int, $userName: String, $mediaId: Int) {
        MediaList(userId: $userId, userName: $userName, mediaId: $mediaId) {
            progressVolumes
            status
            score(format: POINT_10)
            progress
            repeat
            user {
                name
            }
        }
    }";

    let relation_stats: &str = "
    query ($id: Int, $page: Int, $perPage: Int, $search: String, $type: MediaType) {
    Page (page: $page, perPage: $perPage) {
        media (id: $id, search: $search, type: $type) {
            id
            type
            synonyms
            title {
                romaji
                english
                native
                }
            }
        }
    }";

    let affinity: &str = "
    query ($userName: String, $perChunk: Int, $type: MediaType) {
        MediaListCollection(userName: $userName, perChunk: $perChunk, type: $type) {
            user {
                name
                avatar { large }
                siteUrl
            }
            lists {
                entries {
                    status
                    score(format: POINT_100)
                    mediaId

                }
            }
        }
    }
    ";

    let recommendation_amount: &str =  "
        query ($page: Int, $perPage: Int) {
        Page(page: $page, perPage: $perPage) {
            pageInfo {
                hasNextPage,
                lastPage,
            }
            media {
                id
            }
        }
        }";

    let recommendation: &str =  "
        query ($genres: [String], $type: MediaType, $page: Int, $perPage: Int) {
        Page(page: $page, perPage: $perPage) {
            pageInfo {
                hasNextPage,
                lastPage,
            }
            media(type: $type, genre_in: $genres, sort: ID) {
                id
            }
            }
        }";

    match query_name {
        "search" => search.to_string(),
        "user_stats" => user_stats.to_string(),
        "relation_stats" => relation_stats.to_string(),
        "user" => user.to_string(),
        "affinity" => affinity.to_string(),
        "recommendation" => recommendation.to_string(),
        "recommendation_amount" => recommendation_amount.to_string(),
        _ => panic!("Invalid Query Name"),
    }
}