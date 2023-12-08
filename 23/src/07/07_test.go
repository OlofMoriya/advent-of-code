package main

import "testing"

func TestGetHandType(t *testing.T) {
    got := getTypeOfHandB("QTQTT")
    want := FullHouse 

    if got != want {
        t.Errorf("got %q, wanted %q", got, want)
    }

    got = getTypeOfHandB("J4555")
    want = FourOfAKind

    if got != want {
        t.Errorf("got %q, wanted %q", got, want)
    }


    got = getTypeOfHandB("J4444")
    want = FiveOfAKind

    if got != want {
        t.Errorf("got %q, wanted %q", got, want)
    }
    
}

func TestGetThrees(t *testing.T) {
    cards := "6668Q"
    got := getTypeOfHandB(cards)
    want := ThreeOfAKind

    if got != want {
        t.Errorf("got %q, wanted %q, for cards %s", got, want, cards)
    }
}
