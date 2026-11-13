+++
number = 0
label = "B"
title = "Organic chemistry for coffee"
question = "Which functional groups and reaction types do roasting and staling rely on?"
objectives = [
  "Read condensed structural formulas and identify the functional groups in coffee compounds.",
  "Explain why carbonyl groups react with nucleophiles, and describe imine formation, enolisation and the aldol reaction.",
  "Describe the formation and hydrolysis of esters and lactones, using chlorogenic acids as the example.",
  "Describe the oxidation of phenols to quinones and the addition of thiols to quinones.",
  "Recognise the aromatic ring systems of coffee aroma and explain their stability.",
]
+++
Chapter 8 describes roasting in terms of reaction types: condensation, rearrangement, fragmentation, esterification, oxidation. Chapter 5 explains staling the same way. This appendix supplies the organic chemistry behind those descriptions for readers who have not taken a course in it, or took one long ago. It starts from how carbon bonds and builds up only the ideas that the chapters use.

## B.1 Carbon compounds and how to read them

Organic chemistry is the chemistry of carbon compounds. Carbon forms four {{covalent bond|covalent bonds}}, each a pair of electrons shared between two atoms. Hydrogen forms one bond, oxygen two, nitrogen three and sulfur usually two. Two atoms can share two pairs, a {{double bond}}, as in C=O and C=C.

Structures in this book are written in condensed form. Acetic acid is CH<sub>3</sub>COOH: a carbon bearing three hydrogens joined to a carbon that carries a double-bonded oxygen and an OH group. Larger molecules are described in words, naming their groups.

When two atoms share electrons unequally, the bond is {{polar bond|polar}}. Oxygen and nitrogen attract electrons more strongly than carbon and hydrogen do; this attraction is called {{electronegativity}}. In a C=O bond, the oxygen carries a partial negative charge and the carbon a partial positive charge. Most of the reactions in this appendix begin with that imbalance.

## B.2 Functional groups in coffee

A {{functional group}} is a small arrangement of atoms that reacts in characteristic ways wherever it occurs. A molecule's behaviour is largely the sum of its groups.

| Group | Structure | Coffee examples |
|---|---|---|
| Hydroxyl (alcohol) | R&ndash;OH | sugars, quinic acid, furfuryl alcohol |
| Aldehyde | R&ndash;CHO | open-chain glucose, Strecker aldehydes, hexanal, furfural |
| Ketone | R&ndash;CO&ndash;R' | open-chain fructose, diacetyl, Amadori products |
| Carboxylic acid | R&ndash;COOH | acetic, citric, malic, quinic acids |
| Ester | R&ndash;CO&ndash;O&ndash;R' | chlorogenic acids, oils (triglycerides) |
| Lactone (cyclic ester) | ring containing &ndash;CO&ndash;O&ndash; | chlorogenic acid lactones |
| Amine | R&ndash;NH<sub>2</sub> | amino acids, proteins |
| Phenol | OH on a benzene ring | caffeic acid, guaiacol, 4-vinylguaiacol |
| Catechol | two neighbouring OH groups on a benzene ring | caffeic acid, hydroxyhydroquinone |
| Thiol | R&ndash;SH | cysteine, 2-furfurylthiol, methanethiol |
| Thioether | R&ndash;S&ndash;R' | methionine, methional |

The symbols R and R' stand for the rest of the molecule. Aldehydes and ketones both contain the {{carbonyl group}}, C=O; in an aldehyde the carbonyl carbon carries at least one hydrogen, while in a ketone it is bonded to two carbons.

Caffeic acid shows how groups combine. It carries a catechol, a C=C double bond next to the ring, and a carboxylic acid. It can therefore be esterified (forming chlorogenic acids), oxidised (forming quinones), lose CO<sub>2</sub> (forming vinylcatechol) and join larger molecules (forming melanoidins).

## B.3 The carbonyl group

The partial positive charge on the carbonyl carbon attracts {{nucleophile|nucleophiles}}, molecules or groups that carry a pair of electrons available to form a new bond. Water, alcohols and amines are all nucleophiles. Much of roasting chemistry begins with a nucleophile attacking a carbonyl carbon.

### Nucleophilic addition

In {{nucleophilic addition}}, the nucleophile bonds to the carbonyl carbon, and the pair of electrons from the C=O double bond moves onto the oxygen, which then picks up a proton. The carbonyl becomes a carbon bearing an OH group and the newly attached group.

Sugars use this reaction on themselves. Glucose has an aldehyde group at one end and OH groups along its chain. The OH on its fifth carbon attacks its own aldehyde, closing a six-membered ring called a {{hemiacetal}}. In solution, more than 99% of glucose exists as this ring. Only the small open-chain fraction has a free carbonyl, which is why sugars react slowly with amines until heat increases that fraction and the reaction rates.

### Imine formation

When the nucleophile is an amine, R'&ndash;NH<sub>2</sub>, addition gives a carbon bearing both OH and NHR'. This intermediate loses water to form an {{imine}}, which contains a carbon-nitrogen double bond, C=N&ndash;R':

~~~eq imine
formula = "RCHO + R'NH<sub>2</sub> &harr; RCH(OH)NHR' &rarr; RCH=NR' + H<sub>2</sub>O"
where = [["RCHO", "an aldehyde", ""], ["R'NH<sub>2</sub>", "an amine", ""], ["RCH=NR'", "the imine", ""]]
~~~

A little acid speeds the loss of water by adding a proton to the OH group, turning it into water, which leaves easily. Too much acid adds a proton to the amine instead, and a protonated amine is no longer a nucleophile. The reaction is therefore fastest at mildly acidic pH. Imine formation from a sugar and an amino acid is the first step of the Maillard reaction (Chapter 8).

### Enols and tautomers

A carbonyl compound with a hydrogen on the carbon next to the carbonyl can rearrange: the hydrogen moves to the oxygen and a C=C double bond forms. The product, which has an OH on a double-bonded carbon, is an {{enol}}. A pair of forms like this, interconverting by moving a hydrogen and a double bond, are {{tautomer|tautomers}}. The equilibrium usually lies far on the carbonyl side, but the enol is the reactive form for many reactions. The Amadori rearrangement of Chapter 8 passes through an enol before settling as a ketone.

### Aldol reactions and retro-aldol fragmentation

An enol, or its negatively charged form, can attack the carbonyl carbon of a second molecule, forming a new carbon-carbon bond. This {{aldol reaction}} is often followed by loss of water, giving a C=C double bond next to the C=O. Aldol reactions between small sugar fragments build larger, coloured molecules on the way to melanoidins.

The reverse reaction, {{retro-aldol}} fragmentation, splits a sugar between two carbons. It is how six-carbon sugars break into the two- and three-carbon fragments that become acetic acid, methylglyoxal and other small compounds during roasting.

~~~quiz
title = "Check your understanding: Sections B.1 to B.3"
[[q]]
type = "choice"
prompt = "Why is the carbon of a C=O group attacked by nucleophiles?"
options = ["Oxygen draws electrons away, leaving the carbon partly positive", "The carbon carries a negative charge", "Carbon forms only two bonds"]
answer = 0
explain = "Oxygen is more electronegative, so the bond is polar and the carbon is electron-poor."

[[q]]
type = "choice"
prompt = "Why is imine formation slow in strongly acidic solution?"
options = ["Acid destroys the carbonyl group", "The amine picks up a proton and stops being a nucleophile", "Water cannot be lost in acid"]
answer = 1
explain = "A protonated amine has no free electron pair to donate."
~~~

## B.4 Esters and lactones

A carboxylic acid and an alcohol combine to form an {{ester}} and water. The reaction, {{esterification}}, is catalysed by acid and reversible:

~~~eq ester
formula = "RCOOH + R'OH &harr; RCOOR' + H<sub>2</sub>O"
where = [["RCOOR'", "the ester", ""]]
~~~

The reverse, splitting an ester with water, is {{hydrolysis}}. Low water and high temperature during roasting favour esters forming; in the brew, slow hydrolysis can occur.

Chlorogenic acids are esters of quinic acid with hydroxycinnamic acids such as caffeic acid. The common forms, 3-, 4- and 5-caffeoylquinic acid, differ in which of quinic acid's OH groups carries the ester. Heat makes the caffeoyl group move between neighbouring positions, so roasting changes the proportions of the three.

A {{lactone}} is a cyclic ester, formed when one molecule contains both an acid group and an OH group that can reach each other. Quinic acid's carboxylic acid can react with one of its own OH groups, closing a ring and releasing water. In a chlorogenic acid this produces a caffeoylquinide, one of the bitter lactones of medium roasts (Chapters 1 and 8). Lactones hydrolyse back slowly in water at brew pH.

## B.5 Phenols, catechols and quinones

A {{phenol}} has an OH group on a {{benzene ring}}, a flat six-carbon ring whose electrons are shared around the whole ring. Phenols are weak acids (pK<sub>a</sub> about 10) and good antioxidants. The hydrogen of the OH group is easily given to a {{radical}}, a reactive species with an unpaired electron, and the phenol is left as a radical stabilised by the ring.

A catechol, with two neighbouring OH groups, is oxidised by oxygen, especially when metal ions are present, to an {{ortho-quinone}}. Quinones are strongly electron-poor and react readily with nucleophiles. The reaction adds the nucleophile to a carbon of the ring, a process called {{conjugate addition}}. Thiols are excellent nucleophiles for it, which is why 2-furfurylthiol disappears from the brew as hydroxyhydroquinone oxidises (Chapter 5). Amines and amino acids add in the same way, linking phenolic compounds to proteins and melanoidins.

Breakdown of phenolic acids also gives smoky and spicy volatiles. Ferulic acid loses CO<sub>2</sub> to give 4-vinylguaiacol; further breakdown gives guaiacol.

## B.6 Thiols and sulfur

A {{thiol}}, R&ndash;SH, is the sulfur counterpart of an alcohol. Sulfur is larger than oxygen and holds its outer electrons more loosely. As a result, thiols are more acidic than alcohols (pK<sub>a</sub> about 9 to 10), better nucleophiles, and much more easily oxidised. Two thiols oxidise to a {{disulfide}}, R&ndash;S&ndash;S&ndash;R:

~~~eq disulfide
formula = "2 RSH + [O] &rarr; RSSR + H<sub>2</sub>O"
where = [["[O]", "an oxidising agent, such as oxygen", ""]]
~~~

Many thiols have extremely low odour thresholds, so trace amounts matter. 2-Furfurylthiol forms when the furfuryl cation, produced from furfuryl alcohol by loss of water, is attacked by the sulfur of cysteine (Chapter 8). It is lost by oxidation to disulfides, by addition to quinones and by attack from radicals.

## B.7 Aromatic rings

Some rings are unusually stable. A ring is {{aromatic}} when it is flat, every ring atom takes part in a continuous system of shared electrons, and that system contains 2, 6, 10 or, in general, 4n + 2 electrons (Hückel's rule). Benzene is the standard example. Aromatic rings resist reactions that would break the shared system, so once they form during roasting they survive and accumulate.

Rings that contain an atom other than carbon are {{heterocycle|heterocycles}}. Many of coffee's aroma compounds are aromatic heterocycles:

| Ring | Ring atoms besides carbon | Coffee examples | Typical odour |
|---|---|---|---|
| Furan | one oxygen | furfural, furfuryl alcohol, 2-methylfuran | bready, caramel |
| Pyrrole | one nitrogen | 1-methylpyrrole, 2-acetylpyrrole | sweet, nutty |
| Thiophene | one sulfur | 2-methylthiophene | sulfurous |
| Pyridine | one nitrogen (six-membered ring) | pyridine, from trigonelline | pungent |
| Pyrazine | two nitrogens (six-membered ring) | 2,3-dimethylpyrazine | roasty, nutty, earthy |
| Thiazole | one nitrogen and one sulfur | 2-acetylthiazole | nutty, popcorn |

In furan and pyrrole, the oxygen or nitrogen contributes a pair of electrons to the six-electron system. In pyridine and pyrazine, the nitrogen's spare electron pair points outward from the ring and is not part of the system, which lets these rings accept a proton weakly: they are weak bases.

## B.8 Isomers

{{Isomer|Isomers}} are compounds with the same formula but different structures, and they can differ in taste, odour and reactivity.

- Positional isomers differ in where a group sits: 3-, 4- and 5-caffeoylquinic acid.
- Geometric isomers differ in arrangement around a double bond. Natural caffeic and ferulic acids are mostly the trans form, with the large groups on opposite sides; light and heat convert some to the cis form.
- Stereoisomers differ in three-dimensional arrangement at a {{chiral}} carbon, one bonded to four different groups. Quinic acid has several such carbons, and sugars are chiral.

Because isomers can taste different, analytical methods that separate them (Appendix D.6) matter in coffee research.

~~~keyterms
~~~

## Problems

~~~problems
[[p]]
tier = "A"
section = "B.2"
prompt = "Identify all the functional groups in caffeic acid (3,4-dihydroxycinnamic acid)."
solution = "A catechol (two neighbouring phenolic OH groups on a benzene ring), a C=C double bond next to the ring, and a carboxylic acid."

[[p]]
tier = "A"
section = "B.3"
prompt = "Why does glucose react slowly with amino acids at room temperature?"
solution = "Glucose exists almost entirely as a cyclic hemiacetal with no free carbonyl; only a tiny open-chain fraction can react with amines. Heating raises both that fraction and the rate constants."

[[p]]
tier = "A"
section = "B.7"
prompt = "State the ring atoms other than carbon in furan, pyrazine and thiazole, and explain why such rings persist after roasting."
solution = "Furan: one oxygen. Pyrazine: two nitrogens. Thiazole: one nitrogen and one sulfur. All are aromatic, so they are unusually stable and accumulate rather than breaking down."

[[p]]
tier = "B"
section = "B.3"
prompt = "Describe in words the steps from glucose and the amino acid glycine to an imine."
solution = "The open-chain aldehyde of glucose is attacked by the amino group of glycine (nucleophilic addition), giving a carbon that carries both OH and NH. That intermediate loses water, helped by a little acid, to form a C=N double bond: the imine. In the Maillard reaction the imine then closes into a ring and rearranges (Chapter 8)."

[[p]]
tier = "B"
section = "B.3"
prompt = "Explain why imine formation is fastest at mildly acidic pH."
solution = "Acid helps the OH group of the intermediate leave as water by adding a proton to it. But strong acid adds a proton to the amine, which then has no free electron pair and cannot act as a nucleophile. The best rate is a compromise at mildly acidic pH."

[[p]]
tier = "B"
section = "B.4"
prompt = "Describe the formation of a lactone from 3-caffeoylquinic acid. What small molecule is lost?"
solution = "The carboxylic acid group of the quinic acid unit forms an ester with one of the OH groups on the same molecule, closing a ring: a caffeoylquinide. Water is lost."

[[p]]
tier = "B"
section = "B.5"
prompt = "Explain why 2-furfurylthiol is lost faster in a brew containing more oxidised catechols."
solution = "Catechols oxidise to ortho-quinones, which are electron-poor. The thiol adds to the quinone ring by conjugate addition, forming a product that does not evaporate and has no odour. More quinone means faster thiol loss."

[[p]]
tier = "B"
section = "B.3"
prompt = "Retro-aldol fragmentation of fructose (six carbons) between its third and fourth carbons gives two three-carbon fragments. Name them, and say which reactive dicarbonyl they can form."
solution = "Dihydroxyacetone and glyceraldehyde. Either can lose water and rearrange to methylglyoxal, a dicarbonyl that drives the Strecker degradation."

[[p]]
tier = "C"
section = "B.7"
prompt = "Explain why pyrazine is a weaker base than a simple amine such as methylamine."
solution = "In pyrazine the nitrogen's spare electron pair lies in the plane of the ring in an orbital with more s character, held closer to the nucleus, and the second nitrogen in the ring draws electron density away. In methylamine the electron pair is held more loosely and is more available to a proton."

[[p]]
tier = "C"
section = "B.4"
prompt = "Roasting changes the proportions of 3-, 4- and 5-caffeoylquinic acid. Propose a mechanism and say how you would detect it."
solution = "Acyl migration: a neighbouring OH group attacks the ester's carbonyl carbon, forming a short-lived ring intermediate that opens to leave the caffeoyl group on the neighbouring position. Detect it by HPLC with a column that separates the three isomers, comparing green and roasted extracts against standards (Appendix D.6)."
~~~

## Further reading

Clayden J., Greeves N., Warren S., *Organic Chemistry*, chapters on carbonyl chemistry, enols and aromatic heterocycles; Farah A. (ed.) (2019), *Coffee: Production, Quality and Chemistry*, Royal Society of Chemistry, chapters on chlorogenic acids and melanoidins.
