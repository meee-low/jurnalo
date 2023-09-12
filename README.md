# Jurnalo

This is a simple app for personal journaling.

You can set your own questions, prompts etc.

This also works as a mood tracker and habit tracker.


(Work in progress, completely non-functional)



## Mock-up: 


```
>> journalo full
How are you?

[1] great [2] good [3] meh [4] bad [5] terrible [c] custom

> 1
What's the weather like today?
[1] Sunny [2] Cloudy [3] Rainy [4] Snowing [5] Warm [6] Cold ... 

> 1 3 6 
What have you been up to? 
[1] ... // (categories)

> Coding 
Which habits did you complete?
[1] Meditation [2] *Gym [3] Running [4] Reading [5] Cleaning ... // (custom)
> 1 2 : Gym was very intense today, now my legs are sore :( // (comment)

Finished logging today! Here are your current habit streaks:

                 S S M T W T F
Meditation:      #   # #     # 
Gym:               # #       # 
...

>>

```


```
>> journalo log Today was a great day! Had lunch with Monica.
Added "Today was a great day! Had lunch with Monica."
```

```
>> journalo print day|week|month|...
Shows the entries to this period in the stdout.
```

```
>> journalo export day|week|month|... -o filepath
Creates a markdown file with all the entries in the period. 
```

