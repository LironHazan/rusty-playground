-- General notes:

-- Functions that have type variables are called polymorphic functions.
-- Point to think of Result trait is similar to Either typeclass (haskell)


class Song a where -- "a" is a param
    play :: () -> String

instance Song String where
    play (note) = show


sing :: (Song s) =>

read :: (Read a) => String -> a
