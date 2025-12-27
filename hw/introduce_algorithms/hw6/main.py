import sys


def solve():
    jobs = []

    try:
        lines = sys.stdin.readlines()
        for line in lines:
            parts = list(map(int, line.split()))
            if len(parts) >= 3:
                jobs.append({"id": parts[0], "dead": parts[1], "profit": parts[2]})
    except Exception as e:
        pass

    if not jobs:
        return

    max_deadline = 0
    for job in jobs:
        if job["dead"] > max_deadline:
            max_deadline = job["dead"]

    slots = [0] * (max_deadline + 1)

    total_profit = 0
    selected_jobs = []

    for job in jobs:
        for time_slot in range(job["dead"], 0, -1):
            if time_slot <= max_deadline and slots[time_slot] == 0:
                slots[time_slot] = job["id"]
                total_profit += job["profit"]
                break

    final_sequence = []
    for job_id in slots:
        if job_id != 0:
            final_sequence.append(job_id)

    print(f"最佳序列: {final_sequence}")
    print(f"最大利潤: {total_profit}")


if __name__ == "__main__":
    solve()
