def add_parents(graph: dict, d_sep_graph: dict):
    changed = True
    while changed:
        changed = False
        for possible_parent, possible_parent_children in graph.items():
            keys = list(d_sep_graph.keys())
            for node in keys:
                if node in possible_parent_children:
                    if possible_parent not in d_sep_graph.keys():
                        d_sep_graph[possible_parent] = set()
                        changed = True
                    if node not in d_sep_graph[possible_parent]:
                        d_sep_graph[possible_parent].add(node)
                        changed = True


def moralize(d_sep_graph: dict):
    keys = list(d_sep_graph.keys())
    for node1 in keys:
        for node2 in keys:
            if node1 != node2:
                node1_children = list(d_sep_graph[node1])
                for child in node1_children:
                    node_2_children = list(d_sep_graph[node2])
                    if child in node_2_children:
                        d_sep_graph[node1].add(node2)
                        d_sep_graph[node2].add(node1)


def double_edge(d_sep_graph: dict):
    keys = list(d_sep_graph.keys())
    for node1 in keys:
        for node2 in keys:
            if node1 != node2:
                if node1 in d_sep_graph[node2] or node2 in d_sep_graph[node1]:
                    d_sep_graph[node1].add(node2)
                    d_sep_graph[node2].add(node1)


NOT_SEEN = 0
SEEN = 1


def find_path(graph: dict, node, first, finish, found: dict, marked: list):
    if node == finish:
        return True

    if node in marked and node != first:
        return False

    found[node] = SEEN

    for child in graph[node]:
        if found[child] != SEEN:
            if find_path(graph, child, first, finish, found, marked):
                return True

    found[node] = NOT_SEEN

    return False


def d_separate(graph: dict, first, second, deps):
    d_sep_graph = {
        first: set(),
        second: set(),
    }

    for c_ in deps:
        d_sep_graph[c_] = set()

    marked = list(d_sep_graph.keys())

    add_parents(graph, d_sep_graph)
    moralize(d_sep_graph)
    double_edge(d_sep_graph)

    print(d_sep_graph)

    found = {d: NOT_SEEN for d in d_sep_graph.keys()}
    return find_path(d_sep_graph, first, first, second, found, marked)


def main():
    graph = {}
    line = "-"
    while line != "":
        line = input("ny node: ")
        if line != "":
            graph[line] = set()

    print("Nodes:")
    for node in graph.keys():
        print(f"\t{node}")
        graph[node] = set()

    print("Add edges:")
    while True:
        line = input("ny edge: ")
        if len(line.split(" ")) != 2:
            break
        from_node, to_node = line.split(" ")
        graph[from_node].add(to_node)

    print("Edges:")
    for k, v in graph.items():
        for node in v:
            print(f"\t{k} -> {node}")
    quest = input("_ _ | _ _ ...> ").split(" ")

    if len(quest) == 2:
        print(d_separate(graph, quest[0], quest[1], []))
    elif len(quest) >= 4:
        print(d_separate(graph, quest[0], quest[1], quest[3:]))
    else:
        print("Must have 2 or greater than 3 arguments")


if __name__ == '__main__':
    # main()
    print(d_separate({
        "A": {"B"},
        "B": {"C"},
        "C": {},
        "D": {"C"}
    }, "A", "D", ["C", "B"]))
