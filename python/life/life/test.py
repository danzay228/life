
def func(*args):
    func.count = getattr(func, 'count', 0) + 1
    print(func.count, args)


class CClass:
    def func(self, *args):
        self.func.count = getattr(self.func, 'count', 0) + 1
        print(self.func.count, args)


if __name__ == '__main__':
    func(12)
    func(3)
    func(5, 6)

    c = CClass()
    c.func(12)
    c.func(3)
    c.func(5, 6)
