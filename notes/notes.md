If you don't have the plotting script locally, you can get it by adding a code cell and executing this code in it.

```Python
from urllib.request import urlretrieve
URL = 'https://go.gwu.edu/engcomp4plot'
urlretrieve(URL, 'plot_helper.py')
```

##### Note:

Our helper function `plot_vector()` takes one or two lists as arguments: a list of vectors, and a list of tails (optional). It can plot one vector with its tail on several locations, or several vectors with their tail at one location. It can also plot several vectors with their tails at different locations, but in that case, the two lists have to match in length (if they don't, the function will give an error).

