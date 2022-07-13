#Simple CMD Path ShortCut Maker

make a shortcut to help you access path easily

this file can make a bat file :
    
    ``` 
    D: //{ disk of your current path }
    cd { current path }
    ```
###HowToUse:
-save the exe file to directory that you want to save all the bat files
-add that directory to your PATH environment variable
-open your cmd and go to the directory that you want to make shortcut
-type ```defpath new { name of the shortcut }``` to create a new shortcut
-the bat file will be created in the same directory as the exe file
-now you can access the path you saved by typing the name of the shortcut in the cmd 

###Usage:
>defpath new { name of the shortcut } //to create a new shortcut\n
>defoath delete { name of the shortcut } //to delete a shortcut\n
>defpath list //to list all the shortcuts\n


