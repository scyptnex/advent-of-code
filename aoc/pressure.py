import sys
import random


class Pressure:
    def __init__(self):
        self.releasedPressure = 0

    def releasePressure(self, pressureReleased):
        self.releasedPressure += pressureReleased


class Valve:
    def __init__(self, location, flowRate, tunnels):
        self.location = location
        self.flowRate = flowRate
        self.tunnels = tunnels
        self.open = False


class Player:
    def __init__(self, ValveLocation):
        self.location = ValveLocation

    def move(self, newLocation):
         for valve in ValveList:
             if newLocation == valve.location: 
                if self.location.location in valve.tunnels:
                    self.location = valve


def ProcessMovement(playerAction, newLocation=False):
    for valve in ValveList:
        if valve.open == True:
            pressure.releasePressure(valve.flowRate)

    if playerAction == "Valve":
        player.location.open = True
    elif playerAction == "Nothing":
        pass
    else:
        player.move(newLocation)

def CalculateValue(actionList):
    if player.location.open:
        ValveValue = 0
    else:
        ValveValue = player.location.flowRate

    if ValveValue > 0:
        return "Valve"

    movementOptions = actionList[2:]
    movementValue = []
    currentLargestValue = 0
    currentLargestIndex = 0
    currentIndex = 0

    for option in movementOptions:
        for valve in ValveList:
            if option == valve.location:
                if not valve.open:
                    movementValue.append(valve.flowRate)
                else:
                    movementValue.append(0)

                if currentLargestValue < movementValue[-1]:
                    currentLargestValue = movementValue[-1]
                    currentLargestIndex = currentIndex
        currentIndex += 1

    totalmovementValue = 0
    for i in movementValue:
        totalmovementValue += i

    if totalmovementValue == 0:
        return movementOptions[random.randint(0, len(movementValue) -1)]
    else:
        return movementOptions[currentLargestIndex]

ValveList = []
if __name__ == "__main__":
    for line in sys.stdin:
        strings = line[:-1].split(" ")
        location = strings[1]
        flowRate = int(strings[4].split("=")[-1][:-1])
        tunnels = [l[:-1] if "," in l else l for l in strings[9:]]
        ValveList.append(Valve(location, flowRate, tunnels))
        if location == "AA":
            player = Player(ValveList[-1])

pressure = Pressure()


for i in range(0, 30):
    actions = ["Nothing", "Valve"] + player.location.tunnels

    print("Current Location: " + player.location.location)

    whatDo = CalculateValue(actions)
    if whatDo == "Valve":
        ProcessMovement("Valve")
        print("You openned the Valve!")
    else:
        ProcessMovement("Move", whatDo)
        print("You moved to: " + whatDo)

    print("Pressure released is: " + str(pressure.releasedPressure))


