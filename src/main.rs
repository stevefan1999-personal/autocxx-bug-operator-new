use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    #include "extdll.h"
    #include "util.h"
    #include "cbase.h"

    #include "activity.h"
    #include "animation.h"
    #include "basemonster.h"
    #include "cbase.h"
    #include "cdll_dll.h"
    #include "client.h"
    #include "decals.h"
    #include "defaultai.h"
    #include "doors.h"
    #include "effects.h"
    #include "enginecallback.h"
    #include "explode.h"
    #include "extdll.h"
    #include "flyingmonster.h"
    #include "func_break.h"
    #include "game.h"
    #include "gamerules.h"
    #include "hornet.h"
    #include "items.h"
    #include "maprules.h"
    #include "monsterevent.h"
    #include "monsters.h"
    #include "nodes.h"
    #include "plane.h"
    #include "player.h"
    #include "saverestore.h"
    #include "schedule.h"
    #include "scripted.h"
    #include "scriptevent.h"
    #include "soundent.h"
    #include "spectator.h"
    #include "squad.h"
    #include "squadmonster.h"
    #include "talkmonster.h"
    #include "teamplay_gamerules.h"
    #include "trains.h"
    #include "util.h"
    #include "vector.h"
    #include "weapons.h"
    #include "wxdebug.h"

    safety!(unsafe_ffi) // see details of unsafety policies described in the 'safety' section of the book
    generate!("CBaseEntity")
    generate!("CBaseMonster")
    generate!("CBasePlayer")

    // exclude_impls!()
}

fn main() {
    println!("Hello, World!")
}
