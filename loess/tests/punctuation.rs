use loess::punctuation;

punctuation! {
	#[derive(Clone)] (+) not before [=] as pub Plus: doc, Default, PeekFrom, PopFrom, IntoTokens, SimpleSpanned, LocatedAt, ResolvedAt { pub plus }
	#[derive(Clone)] (<<) not before [=] as pub Shl: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub lt0, pub lt1 }
	#[derive(Clone)] (<<=) as pub ShlEq: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt { pub lt0, pub lt1, pub eq }
	#[derive(Clone)] (+++) as pub PlusPlusPlus: doc, Default, PeekFrom, PopFrom, IntoTokens, LocatedAt, ResolvedAt (pub, pub, pub);
}

//TODO: Test behaviour.
