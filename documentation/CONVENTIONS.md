# Conventions

## Preface

The goal of this document is to explain any conventions used in this project. Conventions are unavoidable, and are part of the sort of dialect that coders write in. Some conventions are popular, such as putting source code in a `src\` directory, others are community based, such as conventions used within a language, and some conventions are personal. We all have our own coding idiolect.

All that said, this document seeks to clarify those that I am aware of that are used within this project. Broader conventions, such as general coding and language conventions will be ignored, focusing mostly on the scope of conventions that arise in this project. However, I will occasionally note broader conventions that are used, usually ones that pertain to things that might differ from certain segments of the larger community. 

## Code

### Prefixes

`Bonfire` - This is used as prefix for items that pertain to the project/program as a whole, specifically those that might cause redundancy with other libraries or modules used. An example is the `BonfireFile` structure, which is used as a wrapper for the `File` structure from the standard library, and adds fields relevant to the Bonfire project (i.e. extension, metadata).

`Ultra` - This is used as a prefix for things that pertain to the Nintendo 64 platform. While most parts of the platform will maintain a non-prefixed name (i.e. CPU, RAM), something like an enum of extension might garner the name `UltraFileExtension` to signify that it is the file extensions for the nintendo 64 platform.

The reason Ultra was chosen was that it was the codename for the Nintendo 64 platform, specifically Ultra 64, and provides a more concise (in both length and scope) than Nintendo.