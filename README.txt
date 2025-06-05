
Rust in Shell

Rust in Shell est un shell minimaliste écrit en Rust. Le projet est déjà fonctionnel dans son état actuel et propose plusieurs commandes internes (builtins), la lecture interactive et l’exécution de programmes externes.

Installation et exécution
-------------------------
1. Cloner le dépôt :
   git clone https://github.com/karlexark/shell_in_rust.git
   cd shell_in_rust

2. Compiler :
   cargo build --release

3. Exécuter :
   ./target/release/rust_in_shell   (ou rust_in_shell.exe sous Windows)

Une fois lancé, l’invite "$ " s’affiche et je peux saisir mes commandes.

Architecture
------------
Le code est organisé en trois fichiers principaux dans src/ :

- main.rs :
  • Boucle principale : lecture de la ligne, ajout à l’historique, découpage sur split_whitespace(), dispatch vers les builtins ou cmd_ext.

- builtins.rs :
  • cmd_echo   : affiche les arguments séparés par un espace.
  • cmd_type   : identifie si une commande est builtin ou cherche un fichier du même nom dans PATH.
  • cmd_history: affiche un nombre donné de lignes de l’historique (n = 0 → n’affiche rien).
  • cmd_pwd    : affiche le répertoire courant sous forme de chemin absolu complet.
  • cmd_cd     : change de répertoire ; gère "~" seul ; si le chemin n’existe pas ou qu’il y a trop d’arguments, j’affiche un message.
  • cmd_ls     : liste et trie les fichiers/dossiers du répertoire passé ou du répertoire courant.
  • cmd_ext    : cherche l’exécutable dans les dossiers de PATH et lance run_external.
  • run_external : exécute une commande externe de manière bloquante et affiche une erreur si ça échoue.

- autocompletion.rs :
  • HelpTab struct : implémente rustyline::completion::Completer.
  • search_match : recherche dans les builtins, puis dans le répertoire courant, puis dans PATH si besoin.
  • match_in_a_vec : calcule le plus long préfixe commun d’une liste de chaînes pour compléter jusqu’à ce préfixe.

Commandes internes (builtins)
------------------------------

1. exit
   • exit        : quitte immédiatement le shell.
   • exit 0      : quitte immédiatement le shell.
   • exit X (X≠0) : affiche « X n’est pas un argument reconnu par exit. » et ne quitte pas.
   • exit X Y    : affiche « Trop d’arguments donnés pour l’utilisation d’exit. »

2. echo [texte...]
   • echo Bonjour tout le monde → affiche « Bonjour tout le monde ».
   • Les espaces multiples sont normalisés en un seul espace, car j’utilise split_whitespace() pour découper.

3. type <commande>
   • Si <commande> est l’un de mes builtins (exit, echo, type, history, pwd, cd, ls) → j’affiche « <commande> is a shell builtin ».
   • Sinon, je parcoure chaque dossier de PATH pour trouver un fichier portant ce nom (qu’il soit exécutable ou non) :
     → si je trouve un fichier, j’affiche « <commande> is <chemin complet> » pour le premier rencontré.
   • Sinon, j’affiche « <commande>: not found ».

4. history <n>
   • history            → Si rien n'est précisé alors on affiche tout l'historique en mémoire
   • history x          → si l’historique contient au moins 5 entrées, j’affiche les 5 dernières, dans l’ordre chronologique ascendant (indices len-x à len), si x est plus grand que le nombre ligne en mémoire j'affiche tout
   • history -x         → impossible, j’affiche « -x n’est pas un argument valide pour history. »
   • history x y        → j’affiche « Trop d’arguments donnés pour history, réessayez avec un seul argument. »

5. pwd
   • pwd    → affiche le chemin absolu complet du répertoire courant.
   • pwd x  → affiche « Cette commande ne prend aucun argument en entrée » et n’affiche pas de chemin.

6. cd <chemin>
   • cd ~  	     → direction le home
   • cd /tmp         → change vers /tmp si possible.
   • cd dossierInexistant → « cd: dossierInexistant: No such file or directory ».
   • cd (sans argument) → je fais continue dans la boucle principale, donc je ne change pas de dossier.
   • cd a b          → affiche « Trop d’arguments donnés pour cd » et ne change rien.

7. ls [répertoire]
   • ls                 → liste le contenu du répertoire courant. 
   • ls /etc            → liste /etc trié.
   • ls dossierVide     → si read_dir échoue, j’affiche « ls : impossible de lire de les fichier de dossierVide : <raison> » puis rien.
   • ls a b             → « Trop d’arguments donnés pour ls » et je fais continue.

Autocomplétion
--------------
- Premier Tab :
1. Si le premier mot est différent de cd ou ls, on parcours la liste des builtins pour trouver une ou plusieurs correspondances
2. Si on ne trouve rien ou que le premier mot saisi est cd ou ls, on regarde si dans le dossier actuel il y a une ou plusieurs correspondances
3. Si on ne trouve rien, on regarde dans tous les dossiers accessibles s'il y a une ou plusieurs correspondances 
4. En fonction du nombre de correspondances trouvé : 
	- Aucune correspondance -> rien ne se passe 
	- Une correspondance -> Remplacement immédiat par la correspondance trouvée
	- Plus de 1 correspondance :
		- S'il y a un préfixe commun entre toutes les correspondances -> remplacement immédiat par le préfixe commun 
		- S'il n'y a pas de préfixe commun -> visuellement il ne se passe rien mais le programme enregistre qu'un premier appui de tab avec ce texte a été effectué

-  Deuxième Tab (dans le cas où il y avait plusieurs correspondances) : 
1. Affichage de toutes les correspondances trouvées sur une nouvelle ligne 
2. Réécriture de la ligne précédente sur une nouvelle ligne 


Exécution de commandes externes
-------------------------------
- Si la commande n’est pas un builtin, cmd_ext(argv, &paths) :
  1. Si argv.len() == 0 : return.
  2. Sinon, je parcours &paths (chaque entrée vient de PATH) :
     - full_path = Path::new(dir).join(argv[0]).
     - Si full_path.is_file(), j’appelle run_external(full_path, &argv[1..]), puis je break.
  3. Si aucun dossier de PATH ne contient argv[0], j’affiche « <commande> : programme not found ».
- run_external(program_name, args) : 
  • Command::new(program_name).args(args).status() :
    - Si Ok(_), je ne fais rien, je retourne au prompt.
    - Si Err(e), j’affiche « Erreur lors de l’execution de <program_name> : <e> ».

Exemples d’utilisation
----------------------
$ pwd
/home/utilisateur/RustInShell

$ ls
Cargo.toml
README.txt
src

$ cd src
$ pwd
/home/utilisateur/RustInShell/src

$ echo Salut
Salut

$ type ls
ls is a shell builtin

$ history 2
0  type ls
1  history 2

$ h [Tab]
$ history 

$ e [Tab]
$ e [Tab]
echo  exit
$ e

$ exit 0
[fin du programme]


Limites actuelles (tout ce qui est géré)
-----------------------------------------
- Pas de parsing de quotes : les arguments sont toujours séparés sur split_whitespace().
- Pas de redirections (> ou <), ni de pipes (|), ni de jobs en arrière-plan (&).
- La complétion ne gère pas les dossiers contenant plusieurs milliers de fichiers de manière optimisée (pas de cache).
- Pas d’options avancées (echo -n, ls -l, etc.) pour l’instant.

Évolutions planifiées
---------------------
- Gérer les quotes et préserver les espaces (parser basique).  
- Ajouter des options pour builtins (echo -n, ls -l, etc.).  
- Supportredirections et pipes.  
- Ajouter des tests unitaires pour chaque builtin.  

Rust in Shell est déjà opérationnel pour les tâches de base : navigation, affichage, exécution de commandes externes, historique et autocomplétion. Les évolutions futures seront documentées dans les prochaines versions de ce README.
