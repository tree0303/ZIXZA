class McAgent:
    def __init__(self) -> None:
        self.gamma = 0.9
        self.epsilon = 0.1
        self.alpha = 0.1
        self.action_size = 4
        random_actions = {0: 0.25, 1: 0.25, 2: 0.25, 3: 0.25}
        self.pi = defaultdict(lambda: random_actions)
        self.q = defaultdict(lambda: 0)
        self.memory = []

    def update(self):
        g = 0
        for data in reversed(self.memory):
            state, action, reward = datag = self.gamma * g + reward
            key = (state, action)
            self.q[key] += (g - self.q[key]) * self.alpha
            self.pi[state] = greedy_probs(self.q, state, self.epsilon)

def greedy_probs(q, state, epsilon=0, action_size=4):
    qs = [q[(state, action)] for action in range(action_size)]
    max_action = np.argmax(qs)
    base_prob = epsilon / action_size
    action_probs = {action: base_prob for action in range(action_size)}
    action_probs[max_action] += (1 - epsilon)
    return action_probs