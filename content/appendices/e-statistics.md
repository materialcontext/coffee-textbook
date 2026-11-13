+++
number = 0
label = "E"
title = "Statistics for sensory work and experiments"
question = "How many cups, tasters and repeats do I need before a difference means something?"
objectives = [
  "Summarise data with a mean, standard deviation, standard error and confidence interval.",
  "Use the binomial distribution to analyse discrimination tests.",
  "Choose between paired and independent t-tests, and explain why pairing gains power.",
  "Run and interpret a one-way ANOVA with a follow-up comparison.",
  "Fit a straight line by least squares and read its residuals.",
  "Compute sample sizes and power before running an experiment.",
  "Explain the ideas behind d' and principal component analysis.",
]
+++
Coffee experiments are small, noisy and expensive: a few roasts, a dozen tasters, a handful of brews. Statistics is the set of tools for deciding what such data can and cannot show. Two errors are common. The first is declaring a difference that is only noise; the second is running an experiment too small to detect a difference that is real. This appendix builds the tools to avoid both, assuming no statistics beyond knowing what an average is.

## E.1 Describing a sample

Measurements vary. If we brewed the same recipe forever, the extraction yields would form a {{population}} with some true mean and spread. In practice we have a {{sample}}: a handful of measurements from which to estimate those population values.

For n values x<sub>1</sub>, &hellip;, x<sub>n</sub>, the {{sample mean}} is m<sub>x</sub> = &Sigma;x<sub>i</sub>/n (many texts write it as x with a bar). The spread is measured by the {{sample standard deviation|standard deviation}}:

~~~eq sd
formula = "s = &radic;[&Sigma;(x<sub>i</sub> &minus; m<sub>x</sub>)<sup>2</sup> / (n &minus; 1)]"
where = [["s", "sample standard deviation", "same unit as x"], ["m<sub>x</sub>", "sample mean", ""], ["n", "number of values", ""]]
~~~

The divisor is n &minus; 1 rather than n because the deviations are measured from a mean calculated from the same data, which makes them slightly too small; dividing by n &minus; 1 corrects for this. The variance is s<sup>2</sup>.

The standard deviation describes the spread of individual values. The {{standard error of the mean|standard error}}, SE = s/&radic;n, describes how precisely the mean itself is known (Chapter 0, Equation 0.1).

A {{confidence interval}} gives a range for the population mean. A 95% confidence interval is built so that, if the experiment were repeated many times, 95% of such intervals would contain the true mean:

~~~eq ci
formula = "m<sub>x</sub> &plusmn; t<sub>0.975, n&minus;1</sub> &times; SE"
where = [["t<sub>0.975, n&minus;1</sub>", "value from Student's t distribution with n &minus; 1 degrees of freedom (2.571 for n = 6; approaches 1.96 for large n)", ""]]
~~~

The {{t distribution}} is used instead of the normal distribution because s is itself an estimate; with few data, the interval must be wider to allow for that extra uncertainty.

~~~example Six brews
Problem. Six pour-overs of one recipe give extraction yields of 20.1, 20.6, 19.8, 20.4, 20.9 and 20.2%. Find the mean, standard deviation and 95% confidence interval.

Plan. Apply {eq:sd}, then SE = s/&radic;n and {eq:ci}.

Solution. Mean: 20.33%. Standard deviation: 0.39. Standard error: 0.39/&radic;6 = 0.158. With t = 2.571, the interval is 20.33 &plusmn; 0.41, from 19.93 to 20.74%.

Check. The interval is about &plusmn;0.4 points wide. A recipe change that moves extraction by 0.3 points cannot be confirmed with six brews per recipe.
~~~

## E.2 Probability and the binomial distribution

Discrimination tests (Chapter 1) count successes. If each of n independent trials succeeds with probability p, the number of successes X follows the {{binomial distribution}}:

~~~eq binom
formula = "P(X = k) = C(n, k) p<sup>k</sup> (1 &minus; p)<sup>n&minus;k</sup>, where C(n, k) = n! / [k!(n &minus; k)!]"
where = [["C(n, k)", "number of ways to choose which k of the n trials succeed", ""]]
~~~

A {{hypothesis test}} starts from a {{null hypothesis}}, here that tasters are guessing, and asks how surprising the data would be if it were true. The {{p-value}} is the probability, assuming the null hypothesis, of a result at least as extreme as the one observed. A small p-value, conventionally below the {{significance level}} &alpha; = 0.05, leads us to reject the null hypothesis. In a triangle test the guessing probability is 1/3, so the p-value is P(X &ge; observed) with p = 1/3.

~~~example Twelve trials
Problem. In a triangle test, 8 of 12 assessors pick the odd cup. Is that significant at 0.05?

Plan. Sum {eq:binom} for k = 8 to 12 with p = 1/3.

Solution. P(X &ge; 8) = 0.0188, below 0.05, so the samples are judged different. With 7 correct, P(X &ge; 7) = 0.066, not significant.

Check. The expected number correct under guessing is 12/3 = 4, so 8 is twice what chance predicts; a small p-value is plausible.
~~~

The {{proportion of discriminators}} converts the result into the share of the population that perceives the difference: p<sub>d</sub> = (3p<sub>c</sub> &minus; 1)/2, where p<sub>c</sub> is the proportion correct (Chapter 1, Equation 1.2). Eight of twelve gives p<sub>c</sub> = 0.667 and p<sub>d</sub> = 0.50, with wide uncertainty at this sample size.

~~~widget triangle-power
~~~

## E.3 Comparing two means: t-tests

A {{t-test}} asks whether a difference between two means is large compared with its standard error. There are two versions, and the design of the experiment decides which to use.

Independent samples: two separate groups, such as two coffees each brewed six times. The statistic is t = (m<sub>1</sub> &minus; m<sub>2</sub>)/SE<sub>diff</sub>, where SE<sub>diff</sub> combines the standard errors of both groups.

Paired samples: each subject provides a value under both conditions, such as each taster scoring both cups. Compute each subject's difference d<sub>i</sub>, then test whether the mean difference is zero:

~~~eq paired
formula = "t = m<sub>d</sub> / (s<sub>d</sub> / &radic;n)"
where = [["m<sub>d</sub>", "mean of the differences", ""], ["s<sub>d</sub>", "standard deviation of the differences", ""], ["n", "number of pairs; degrees of freedom n &minus; 1", ""]]
~~~

~~~example Why pairing matters
Problem. Eight tasters score sweetness for brews A and B on a 0 to 10 scale. Analyse the data as paired and as independent samples.

| Taster | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|
| A | 7 | 6 | 8 | 5 | 7 | 6 | 7 | 8 |
| B | 6 | 6 | 6 | 4 | 6 | 5 | 7 | 6 |

Plan. For the paired test, compute each taster's difference and apply {eq:paired}. For comparison, apply the independent test to the same numbers.

Solution. The differences are 1, 0, 2, 1, 1, 1, 0, 2: mean 1.00, standard deviation 0.756. Paired: t = 1.00/(0.756/&radic;8) = 3.74 with 7 degrees of freedom, p = 0.007. Independent: t = 2.08 with 14 degrees of freedom, p = 0.057.

Check. Tasters use the scale differently: taster 3 scores high and taster 4 low. Pairing removes that variation between tasters and leaves the consistent difference within each taster. A design in which every taster judges both samples is usually more powerful, and the analysis must match the design.
~~~

~~~quiz
title = "Check your understanding: Sections E.1 to E.3"
[[q]]
type = "choice"
prompt = "To describe how variable individual espresso shots are, report the"
options = ["standard deviation", "standard error of the mean", "p-value"]
answer = 0
explain = "The standard deviation describes individual values; the standard error describes the precision of the mean."

[[q]]
type = "numeric"
prompt = "Ten tasters score bitterness for two brews; the differences have mean 0.8 and standard deviation 1.2. What is the paired t statistic?"
answer = 2.11
tolerance = 0.02
unit = ""
explain = "t = 0.8/(1.2/&radic;10) = 2.11."
~~~

## E.4 More than two groups: analysis of variance

Comparing three or more means with repeated t-tests inflates the chance of a false positive: ten tests at &alpha; = 0.05 have a 40% chance of at least one false positive when nothing differs. {{Analysis of variance}} (ANOVA) tests all groups at once.

The idea is to split the total variation in the data into two parts: variation between the group means, and variation within the groups. With k groups and N observations in all,

~~~eq anova
formula = "F = [SS<sub>between</sub> / (k &minus; 1)] / [SS<sub>within</sub> / (N &minus; k)]"
where = [["SS<sub>between</sub>", "sum of squared deviations of group means from the grand mean, weighted by group size", ""], ["SS<sub>within</sub>", "sum of squared deviations of each value from its group mean", ""], ["F", "ratio of between-group to within-group variance", ""]]
~~~

If all groups share one mean, F is close to 1. A large F means the group means differ by more than the noise within groups can explain. The p-value comes from the F distribution with (k &minus; 1, N &minus; k) degrees of freedom.

~~~widget anova
~~~

~~~example Three roasts
Problem. Six panellists score the sweetness of three roasts (the calculator's default data). Do the roasts differ, and which pairs differ?

Plan. Run a one-way ANOVA; if it is significant, use Tukey's test, a {{post-hoc test}} that compares every pair while keeping the overall false-positive rate at 0.05.

Solution. Means are 6.67, 5.17 and 7.83. F = 17.9 with (2, 15) degrees of freedom, p = 0.0001. Tukey's test finds every pair different at 0.05 (p = 0.011, 0.049 and below 0.001).

Check. Because the same six panellists scored each roast, a two-way ANOVA with panellist as a second factor would remove variation between panellists and be more powerful, for the reason shown in Section E.3.
~~~

A two-way ANOVA adds a second factor (panellist, session, brewing method). It can estimate an {{interaction}}, which tests whether the effect of one factor depends on the level of the other. In descriptive analysis, a panellist-by-sample interaction tests whether panellists agree on the ranking of samples, a standard check of panel performance.

## E.5 Regression

{{Linear regression}} fits a straight line y = a + bx to paired data. {{Least squares}} chooses a and b to minimise the sum of the squared vertical distances from the points to the line:

~~~eq ls
formula = "b = &Sigma;(x<sub>i</sub> &minus; m<sub>x</sub>)(y<sub>i</sub> &minus; m<sub>y</sub>) / &Sigma;(x<sub>i</sub> &minus; m<sub>x</sub>)<sup>2</sup>, a = m<sub>y</sub> &minus; b m<sub>x</sub>"
where = [["b", "slope", "unit of y per unit of x"], ["a", "intercept", "unit of y"], ["m<sub>x</sub>, m<sub>y</sub>", "means of x and y", ""]]
~~~

R<sup>2</sup> is the fraction of the variation in y that the line explains. The {{residual|residuals}}, observed minus fitted values, are the main diagnostic: they should scatter without pattern. A curve in the residuals means a straight line is the wrong model, even when R<sup>2</sup> is high. The uncertainty of a slope fitted to n evenly spaced points with noise &sigma; is &sigma;/&radic;&Sigma;(x<sub>i</sub> &minus; m<sub>x</sub>)<sup>2</sup>, which Chapter 7 uses to smooth rate of rise.

~~~example Titratable acidity against strength
Problem. Five brews of one coffee give the following data. Fit titratable acidity against TDS and examine the residuals.

| TDS (%) | 1.10 | 1.20 | 1.30 | 1.40 | 1.50 |
|---|---|---|---|---|---|
| TA (mmol/L) | 1.32 | 1.45 | 1.58 | 1.66 | 1.81 |

Plan. Apply {eq:ls}, then compute fitted values and residuals.

Solution. The fit is TA = 0.017 + 1.19 &times; TDS, with R<sup>2</sup> = 0.994 and a slope standard error of 0.055. The intercept's standard error is 0.072, so the intercept cannot be distinguished from zero. The residuals are &minus;0.006, 0.005, 0.016, &minus;0.023 and 0.008.

Check. The residuals are small and show no trend, and an intercept near zero is consistent with titratable acid being proportional to dissolved solids (Chapter 2A, Section 6).
~~~

The {{correlation coefficient}} r measures linear association on a scale from &minus;1 to 1. It says nothing about cause. In brewing data, two variables often correlate because both depend on a third: strength and bitterness both rise as the brew ratio falls.

## E.6 Power and sample size

{{Statistical power}} is the probability that an experiment detects a real effect of a stated size. Experiments with low power mostly return "no significant difference" whatever the truth, and the significant results they do produce overstate the effect.

For comparing two means, the {{effect size}} d is the difference divided by the standard deviation. The number of replicates needed per group is approximately

~~~eq n
formula = "n &asymp; 2 [(z<sub>1&minus;&alpha;/2</sub> + z<sub>power</sub>) / d]<sup>2</sup>"
where = [["z<sub>1&minus;&alpha;/2</sub>", "normal quantile for the significance level: 1.96 for &alpha; = 0.05", ""], ["z<sub>power</sub>", "normal quantile for the desired power: 0.84 for 80%", ""], ["d", "effect size: difference divided by standard deviation", ""]]
~~~

For &alpha; = 0.05 and 80% power, n &asymp; 15.7/d<sup>2</sup>. A large effect (d = 0.8) needs about 25 per group; a medium effect (d = 0.5), about 63. For a paired design the same formula applies without the leading 2, with d defined as the mean difference divided by the standard deviation of the differences: d = 0.5 needs about 32 pairs.

~~~widget power
~~~

For triangle tests, power depends on the proportion of discriminators. With 24 trials and a true p<sub>d</sub> of 0.30, 13 correct answers are needed for significance and the power is 0.55: almost half the time, a difference that 30% of people can perceive will be missed. ISO 4120 provides tables of sample sizes for chosen error rates and p<sub>d</sub>.

## E.7 Signal detection and d'

The proportion of correct answers depends on the test method as well as on the size of the sensory difference: the same pair of coffees gives different proportions correct in a triangle test and in a two-alternative forced choice. {{Thurstonian model|Thurstonian models}} separate the two. They describe the sensory difference as {{d prime|d'}}, the distance between the two samples' distributions of perceived intensity, measured in units of their standard deviation. For the two-alternative forced choice, d' = &radic;2 z(p<sub>c</sub>), where z(p<sub>c</sub>) is the normal quantile of the proportion correct; 75% correct corresponds to d' = 0.95. The triangle test's relation is more complex and is read from published tables. Because d' does not depend on the test method, it allows results from different tests and laboratories to be compared.

## E.8 Many attributes at once: principal component analysis

A descriptive panel produces a table of samples by attributes. {{Principal component analysis}} (PCA) finds new axes, each a weighted combination of the original attributes, that capture as much as possible of the variation between samples. The first two components often capture most of it and can be drawn as a map: samples that taste alike sit close together, and arrows for the attributes show what separates them. PCA describes structure; it does not test hypotheses, and its axes need interpretation.

~~~keyterms
~~~

## Problems

~~~problems
[[p]]
tier = "A"
section = "E.1"
prompt = "Explain the difference between a standard deviation and a standard error, and say which you would report to describe how variable a grinder's shots are."
solution = "The standard deviation describes the spread of individual values; the standard error describes the uncertainty of the mean and falls as 1/&radic;n. Shot-to-shot variability is a property of individual shots, so report the standard deviation."

[[p]]
tier = "A"
section = "E.2"
prompt = "In a triangle test, 9 of 30 assessors are correct. Is there evidence of a difference? What is the estimated proportion of discriminators?"
solution = "Guessing predicts 10 of 30; 9 is below that, so there is no evidence of a difference (p &gt; 0.5). p<sub>c</sub> = 0.30 gives p<sub>d</sub> = &minus;0.05, which is taken as zero."

[[p]]
tier = "A"
section = "E.4"
prompt = "Why does running ten t-tests at &alpha; = 0.05 on data with no real differences give about a 40% chance of a false positive?"
solution = "If the tests are independent, the probability of no false positive is 0.95<sup>10</sup> = 0.599, so the probability of at least one is 0.401."

[[p]]
tier = "B"
section = "E.1"
prompt = "Five shots give extraction yields of 19.2, 19.8, 18.9, 19.5 and 19.6%. Compute the mean, the standard deviation and a 95% confidence interval (t<sub>0.975, 4</sub> = 2.776)."
solution = "Mean 19.40%. Deviations &minus;0.2, 0.4, &minus;0.5, 0.1, 0.2; their squares sum to 0.50; s = &radic;(0.50/4) = 0.354. SE = 0.158. Interval: 19.40 &plusmn; 0.44, from 18.96 to 19.84%."

[[p]]
tier = "B"
section = "E.3"
prompt = "Ten tasters score bitterness for two brews; the differences have mean 0.8 and standard deviation 1.2. With 9 degrees of freedom the two-sided 5% critical value of t is 2.262. Is the difference significant?"
solution = "t = 0.8/(1.2/&radic;10) = 0.8/0.379 = 2.11, below 2.262, so the difference is not significant at 0.05."

[[p]]
tier = "B"
section = "E.6"
prompt = "How many shots per grinder are needed to detect a difference of 0.4 points in extraction yield, if the shot-to-shot standard deviation is 0.5, at &alpha; = 0.05 with 80% power?"
solution = "d = 0.8; n &asymp; 2(2.80/0.8)<sup>2</sup> = 24.5, so 25 shots per grinder."

[[p]]
tier = "B"
section = "E.5"
prompt = "A regression of sourness score on TDS gives a high R<sup>2</sup>, but the residuals are positive at both ends and negative in the middle. What does this mean, and what would you try?"
solution = "The relationship curves: a straight line is the wrong model over this range. Try adding a squared term or transforming a variable, or add extraction yield, which Chapter 2A suggests also drives sourness."

[[p]]
tier = "B"
section = "E.7"
prompt = "A two-alternative forced choice gives 84% correct. Estimate d', using z(0.84) = 0.994."
solution = "d' = &radic;2 &times; 0.994 = 1.41."

[[p]]
tier = "C"
section = "E.3"
prompt = "Design a test of whether a new water recipe changes perceived sourness in filter coffee. Specify the sensory method, number of assessors, controls for order and temperature, and the analysis."
solution = "Run a triangle test first to establish whether there is any difference, then a paired rating design in which each assessor rates both waters, brewed from one coffee and grinder on the same day. Randomise and balance serving order, code cups with random three-digit numbers, and serve at a fixed temperature. For a paired design aiming at d = 0.5, about 32 assessors are needed (or 16 assessors over two sessions, analysed with assessor and session as factors). Analyse with a paired t-test or a two-way ANOVA with assessor as a factor, and measure titratable acidity and TDS of each brew to check the chemistry."

[[p]]
tier = "C"
section = "E.6"
prompt = "A café ran a triangle test with 6 staff and found no significant difference between its old and new grinder. The manager concludes the grinders are equivalent. Critique the conclusion."
solution = "With 6 trials, 5 correct are needed for significance, and if 30% of people could perceive the difference the probability of 5 or more correct is only about 0.14. Failing to reject the null hypothesis is not evidence of equivalence. Showing equivalence needs a larger test designed with a stated maximum acceptable p<sub>d</sub> and high power (ISO 4120 similarity testing), or an instrumental comparison of particle size and extraction variability."
~~~

## Further reading

Lawless H.T., Heymann H., *Sensory Evaluation of Food: Principles and Practices*; ISO 4120:2021, *Sensory analysis, methodology, triangle test*; Montgomery D.C., *Design and Analysis of Experiments*.
