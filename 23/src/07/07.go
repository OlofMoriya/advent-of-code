package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func FileToStrings(year string, day string, test bool) []string {
    suffix := ""
    if test {
        suffix = "_test"
    }

    bytes, err := os.ReadFile("/Users/olofmoriya/versioned/personal/advent-of-code/" + year +"/input/" + year + "_" + day + suffix)

    if err != nil {
        fmt.Println("Failed to read file", err)
        os.Exit(1)
    }
    
    all_strings := strings.Split(string(bytes), "\n")
    all_strings = all_strings[:len(all_strings)-1]
    return all_strings
}

type TypeOfHand int
const (
    HighCard TypeOfHand = iota
    Pair 
    TwoPairs
    ThreeOfAKind
    FullHouse
    FourOfAKind
    FiveOfAKind
)

type Hand struct {
    Cards string
    Bet int
    TypeOfHand TypeOfHand
}

func getCardValueB(card rune) int {
    return FindIndex([]rune{'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A' }, card)
}

func getCardValue(card rune) int {
    return FindIndex([]rune{ '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A' }, card)
}

func FindIndex(slice []rune, value rune) int {
	for index, item := range slice {
		if item == rune(value) {
			return index // Found the value
		}
	}
	return -1 // Value not found in the slice
}

func a() {
	lines := FileToStrings("23", "07", false)

    // fmt.Println(lines)
    hands := []Hand{}

    for _, line := range lines {
        parts := strings.Split(line, " ")
        hand := parts[0]
        bet,_ := strconv.Atoi(parts[1])
        
        t := getTypeOfHand(hand)

        hands = append(hands, Hand{Cards: hand, Bet: bet, TypeOfHand: t})
    }

    // fmt.Println(hands)
    
    sort.Slice(hands, func(i, j int) bool {
        if hands[i].TypeOfHand == hands[j].TypeOfHand {
            for x := range hands[i].Cards {
                a := getCardValue(rune(hands[i].Cards[x]))
                b := getCardValue(rune(hands[j].Cards[x]))
                if a > b{
                    return false
                } else if a < b{
                    return true
                }
            }
        }
        return hands[i].TypeOfHand < hands[j].TypeOfHand
    })

    var score float64 = 0
    for i,hands := range hands {
         fmt.Println(hands.Cards)
        score += float64((i+1) * hands.Bet)
    }
    
    fmt.Println("score", score)
}

func main() {
	lines := FileToStrings("23", "07", false)
    hands := []Hand{}
    for _, line := range lines {
        parts := strings.Split(line, " ")
        hand := parts[0]
        bet,_ := strconv.Atoi(parts[1])
        
        t := getTypeOfHandB(hand)

        hands = append(hands, Hand{Cards: hand, Bet: bet, TypeOfHand: t})
    }

    // fmt.Println(hands)
    
    sort.Slice(hands, func(i, j int) bool {
        if hands[i].TypeOfHand == hands[j].TypeOfHand {
            for x := range hands[i].Cards {
                a := getCardValueB(rune(hands[i].Cards[x]))
                b := getCardValueB(rune(hands[j].Cards[x]))
                if a > b{
                    return false
                } else if a < b{
                    return true
                }
            }
        }
        return hands[i].TypeOfHand < hands[j].TypeOfHand
    })

    score := 0
    for i,hand := range hands {
        fmt.Println(hand.Cards, hand.TypeOfHand)// hand.Bet, (i+1), (i+1) * hand.Bet)
        score += (i+1) * hand.Bet
    }
    
    fmt.Println("score", score)
}

func getTypeOfHandB(hand string) TypeOfHand {
        cards := map[rune]int{}
        jokers := 0
        for _,card := range hand {
            _, ok := cards[card]
            if card == 'J' {
                jokers++
            } else if ok {
                cards[card] += 1
            } else {
                cards[card] = 1
            }
        }

        highest := 1
        two := false
        twopairs := false


        if jokers == 5 {
            fmt.Println("Five J")
            return FiveOfAKind
        }

        for _, val := range cards {
            if val == 5 {
                highest = max(highest, 4)
            }
            if val == 4 {
                highest = max(highest, 4)
            }
            if val == 3 {
                highest = max(highest, 3)
                if two {
                    twopairs = true
                }
                two = true
            }
            if val == 2 {
                highest = max(highest, 2)
                if two {
                    twopairs = true
                }
                two = true
            }
        }

        if highest + jokers == 5 {
            return FiveOfAKind
        } else if highest + jokers == 4 {
            return FourOfAKind
        } else if highest + jokers == 3 && twopairs {
            return FullHouse
        } else if highest + jokers == 3 {
            return ThreeOfAKind
        } else if twopairs {
            return TwoPairs
        } else if highest + jokers == 2 {
            return Pair
        }

        return HighCard
}

func getTypeOfHand(hand string) TypeOfHand {
        cards := map[rune]int{}

        for _,card := range hand {
            _, ok := cards[card]
            if ok {
                cards[card] += 1
            } else {
                cards[card] = 1
            }
        }

        three := false
        two := false
        twopairs := false

        for _, val := range cards {
            if val == 5 {
                return FiveOfAKind
            }
            if val == 4 {
                return FourOfAKind
            }
            if val == 3 {
                three = true
            }
            if val == 2 {
                if two {
                    twopairs = true
                }
                two = true
            }
        }

        if two && three {
            return FullHouse
        }
        if three {
            return ThreeOfAKind
        }
        if twopairs {
            return TwoPairs
        }
        if two {
            return Pair
        }
        return HighCard
}
