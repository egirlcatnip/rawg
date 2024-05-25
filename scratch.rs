fn main() {

    let rawg = rawg::instance();


    let games = rawg.games();

    let minecraft = rawg.games().from_slug("minecraft".to_string());


}


/*
games().list() ->  Vec[game1,game2,game3...]


games().list().iter().filter(|game| game.rating>= 2.5 )

list()


*/