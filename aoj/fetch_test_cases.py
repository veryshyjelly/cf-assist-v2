from requests import get, post
from urllib.parse import urlparse, parse_qs
from tqdm import tqdm

print("Fetching problem url....")
try:
    problem = get(url="http://localhost:27121/").json()
    url = urlparse(problem['url'])
    print(f"Got Problem url ✅: {problem['url']}")
except:
    print("Cannot fetch problem url from cp-assist 🚫")
    exit(1)

queries = parse_qs(url.query)

if 'id' not in queries:
    print("ID not found 😵")
    exit(1)

id = queries['id'][0]

print("Fetching problem details....")
try:
    headers = get(f"https://judgedat.u-aizu.ac.jp/testcases/{id}/header").json()['headers']
except:
    print("Cannot fetch problem details 😩")
    exit(1)

print(f"Got {len(headers)} test cases 🧶")

test_cases = []
for header in tqdm(headers, desc="Fetching test cases: "):
    serial = header['serial']
    test_case_url = f"https://judgedat.u-aizu.ac.jp/testcases/{id}/{serial}"
    test_case = get(test_case_url).json()
    test_cases.append({'input': test_case['in'], 'output': test_case['out']})

post_result = post("http://localhost:27121/test_cases", json=test_cases)
if post_result.status_code == 200:
    print("Sucessfully added test cases ✅")
else:
    print(f"Failed to add test cases with status code: {post_result.status_code}")