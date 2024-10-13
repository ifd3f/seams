---
name: Spicy Sichuan Stir-Fried Noodles
slug: spicy-sichuan-stir-fried-noodles
equipment:
  grinder:
    recommended: motorized coffee grinder
  wok:
    recommended: steel wok
  spatula:
    recommended: wooden wok spatula
  noodle-pot: {}
  knife: {}
  colander: {}

steps:
  garlicWater:
    title: Garlic water
    note: Mince garlic and immerse in small amount of water to extract flavor and set aside.
    type: preparation
    duration:
      active: 2m
      inactive: "> 10m"
    ingredients:
      - item: garlic
        qt: [6, cloves]

  spiceMixture:
    title: Spice mixture
    note: Grind into a fine powder.
    type: preparation
    duration:
      active: 1m
    ingredients:
      - item: dried asian chili pepper
        qt: [10, peppers]
      - item: sichuan peppercorn
        qt: [20, mL]
    equipment:
      - grinder

  slicedCelery:
    title: Sliced celery
    note: You may cut into horseshoes or any other shape you would like.
    type: preparation
    duration:
      active: 3m
    ingredients:
      - item: celery
        qt: [1, bundle]
    equipment:
      - knife

  washedBeanSprouts:
    title: Washed Bean sprouts
    type: preparation
    duration:
      active: 3m
    ingredients:
      - item: bean sprouts
        qt: [1, bag]
    equipment:
      - colander

  boiledNoodles:
    title: Boiled noodles
    note: Strain noodles afterwards.
    type: preparation
    duration:
      active: 3m
      inactive: 15m
    ingredients:
      - item: noodles
        qt: [3, bundles]
      - item: water
        qt: [1, L]
    equipment:
      - noodle-pot
      - colander

  wokOil:
    title: Heat and oil wok
    note: High heat
    duration:
      active: 1m
    ingredients:
      - item: canola oil
        qt: [1, cup]
    equipment:
      - wok

  wokSpices:
    title: Add spices to wok
    note: >
      Keep wok on high for this step. You may need to stir the spices a little. Do NOT let the spices burn!
    duration:
      active: 1m
    after:
      - wokOil
    ingredients:
      - step: spice-mixture
      - item: minced ginger
        qt: [3, mL]
    equipment:
      - wok
      - spatula

  wokBeef:
    title: Add pseudobeef to wok
    note: >
      Continue to keep heat on high, stirring vigorously.
    duration:
      active: 5m
    after:
      - wokSpices
    ingredients:
      - step: spiceMixture
      - item: impossible beef
        qt: [2, patties]
    equipment:
      - wok
      - spatula

  wokCelery:
    title: Add celery to wok
    note: >
      Continue to keep heat on high, stirring vigorously. To keep celery crunchy, do not spend too long doing this.
    duration:
      active: 2m
    after:
      - wokBeef
    ingredients:
      - step: slicedCelery
    equipment:
      - wok
      - spatula

  wokBeanSprouts:
    title: Add bean sprouts to wok
    duration:
      active: 1m
    after:
      - wokCelery
    ingredients:
      - step: washedBeanSprouts
    equipment:
      - wok
      - spatula

  wokFinishing:
    title: Add noodles and garlic water to wok, and finish
    notes: Turn heat down to medium-low and toss noodles with the toppings.
    result: true
    duration:
      active: 2m
    after:
      - wokBeanSprouts
    ingredients:
      - step: boiledNoodles
      - step: garlicWater
      - item: soy sauce
        qt: [1, cup]
      - item: sesame oil
        qt: [1, cup]
    equipment:
      - wok
      - spatula
# original recollection of recipe:
# > query: today's recipe
# > response:
# >
# > PREPARATION ITEMS (non dependent, any order. chef is advised to pipeline steps in as necessary)
# > 1. 6 cloves minced garlic in small bowl of water to extract flavor, set aside. preferably done first.
# > 2. chilis (~10) and peppercorns (estimation: on order of 100, highly imprecise quantification) in motorized spice grinder. turned into fine powder.
# > 3. celery washed and sliced into horseshoes
# > 4. mung bean sprouts washed and colandered
# > 5. noodles boiled and colandered
# >
# > COOKING
# > 1. large quantity of oil in wok, 3 cubes of ginger and ground spice, heated to extract flavor but not burnt
# > 2. impossible beef, 2 patties, thawed, added, and split up, fried for 5 minutes on high
# > 3. celery added, fried for 2 minutes on high
# > 4. garlic water added
# > 5. mung beans and noodles added, tossed and fried for 1 minute on high
# > 6. soy sauce and sesame oil added to taste
# >
# > recipe is approximate and based on recollection of events, steps, and requirements.
---
